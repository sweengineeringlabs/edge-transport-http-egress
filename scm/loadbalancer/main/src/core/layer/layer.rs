//! Impl blocks for [`LoadbalancerLayerPoolMetrics`] — constructor,
//! [`reqwest_middleware::Middleware`] impl, and [`PoolMetrics`] impl.

use std::sync::Arc;

use async_trait::async_trait;

use crate::api::{
    BackendCountRequest, BackendCountResponse, LoadbalancerConfig, LoadbalancerLayerPoolMetrics,
    LoadbalancerMiddlewareError, PoolMetrics,
};
use swe_edge_loadbalancer::{LoadbalancerSvc, Outcome};

impl LoadbalancerLayerPoolMetrics {
    pub(crate) fn new(config: LoadbalancerConfig) -> Result<Self, LoadbalancerMiddlewareError> {
        let pool = LoadbalancerSvc::build_pool(config)
            .map_err(|e| LoadbalancerMiddlewareError::PoolBuildFailed(e.to_string()))?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// Rewrite the request URL to use the selected backend's origin while
    /// keeping the original path, query, and fragment.
    fn rewrite_url(
        orig: &reqwest::Url,
        backend_url: &str,
    ) -> Result<reqwest::Url, LoadbalancerMiddlewareError> {
        let mut base = reqwest::Url::parse(backend_url)
            .map_err(|e| LoadbalancerMiddlewareError::InvalidBackendUrl(e.to_string()))?;
        base.set_path(orig.path());
        base.set_query(orig.query());
        base.set_fragment(orig.fragment());
        Ok(base)
    }
}

impl std::fmt::Debug for LoadbalancerLayerPoolMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadbalancerLayerPoolMetrics")
            .field("pool", &self.pool)
            .finish()
    }
}

impl PoolMetrics for LoadbalancerLayerPoolMetrics {
    fn backend_count(
        &self,
        _request: BackendCountRequest,
    ) -> Result<BackendCountResponse, LoadbalancerMiddlewareError> {
        Ok(BackendCountResponse {
            value: LoadbalancerSvc::backend_count(&self.pool),
        })
    }
}

#[async_trait]
impl reqwest_middleware::Middleware for LoadbalancerLayerPoolMetrics {
    async fn handle(
        &self,
        mut req: reqwest::Request,
        ext: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let backend = LoadbalancerSvc::select(&self.pool)
            .map_err(|e| reqwest_middleware::Error::Middleware(anyhow::anyhow!("{e}")))?;

        let new_url = Self::rewrite_url(req.url(), &backend.url)
            .map_err(|e| reqwest_middleware::Error::Middleware(anyhow::anyhow!("{e}")))?;
        *req.url_mut() = new_url;

        let backend_id = backend.id.clone();
        // Expose the selected backend to outer layers (e.g. a circuit-breaker
        // above this in the chain) so they can report pool outcomes keyed to
        // the correct backend.
        ext.insert(backend_id.clone());
        let result = next.run(req, ext).await;

        let outcome = match &result {
            Ok(resp) if resp.status().is_server_error() => Outcome::Failure {
                reason: resp.status().to_string(),
            },
            Ok(_) => Outcome::Success,
            Err(e) => Outcome::Failure {
                reason: e.to_string(),
            },
        };
        LoadbalancerSvc::report_outcome(&self.pool, &backend_id, outcome);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_loadbalancer::{BackendConfig, Strategy};

    fn two_backend_config() -> LoadbalancerConfig {
        LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends: vec![
                BackendConfig {
                    url: "https://api-1.internal".to_string(),
                    weight: 1,
                },
                BackendConfig {
                    url: "https://api-2.internal".to_string(),
                    weight: 1,
                },
            ],
        }
    }

    /// @covers: new
    #[test]
    fn test_new_builds_layer_from_valid_config() {
        let layer =
            LoadbalancerLayerPoolMetrics::new(two_backend_config()).expect("valid config builds");
        assert!(format!("{layer:?}").contains("LoadbalancerLayerPoolMetrics"));
        // Sibling negative: an empty backend list must not build a pool.
        let empty = LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends: vec![],
        };
        assert!(LoadbalancerLayerPoolMetrics::new(empty).is_err());
    }

    /// @covers: new
    #[test]
    fn test_new_fails_for_empty_backends() {
        let cfg = LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends: vec![],
        };
        let err = LoadbalancerLayerPoolMetrics::new(cfg).unwrap_err();
        assert!(matches!(
            err,
            LoadbalancerMiddlewareError::PoolBuildFailed(_)
        ));
    }

    /// @covers: backend_count
    #[test]
    fn test_backend_count_reports_pool_size() {
        let layer =
            LoadbalancerLayerPoolMetrics::new(two_backend_config()).expect("valid config builds");
        let count = layer
            .backend_count(BackendCountRequest)
            .expect("infallible")
            .value;
        assert_eq!(count, 2);
        // Sibling case: a single-backend pool reports 1, proving the count is
        // read from the real pool, not a constant.
        let single = LoadbalancerLayerPoolMetrics::new(LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends: vec![BackendConfig {
                url: "https://only.internal".to_string(),
                weight: 1,
            }],
        })
        .expect("valid");
        assert_eq!(single.backend_count(BackendCountRequest).unwrap().value, 1);
    }

    /// @covers: rewrite_url
    #[test]
    fn test_rewrite_url_keeps_path_and_query() {
        let orig = reqwest::Url::parse("https://api.example.com/v1/users?page=2#top").unwrap();
        let rewritten =
            LoadbalancerLayerPoolMetrics::rewrite_url(&orig, "https://api-1.internal:9000")
                .unwrap();
        assert_eq!(rewritten.host_str(), Some("api-1.internal"));
        assert_eq!(rewritten.port(), Some(9000));
        assert_eq!(rewritten.path(), "/v1/users");
        assert_eq!(rewritten.query(), Some("page=2"));
        assert_eq!(rewritten.fragment(), Some("top"));
    }

    /// @covers: rewrite_url
    #[test]
    fn test_rewrite_url_uses_backend_scheme() {
        let orig = reqwest::Url::parse("https://api.example.com/path").unwrap();
        let rewritten =
            LoadbalancerLayerPoolMetrics::rewrite_url(&orig, "http://internal-api").unwrap();
        assert_eq!(rewritten.scheme(), "http");
    }

    /// @covers: rewrite_url
    #[test]
    fn test_rewrite_url_fails_for_invalid_backend_url() {
        let orig = reqwest::Url::parse("https://api.example.com/path").unwrap();
        let err =
            LoadbalancerLayerPoolMetrics::rewrite_url(&orig, "not a url :// !!!").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("invalid backend URL"), "{msg}");
    }

    /// `LoadbalancerLayerPoolMetrics` must be usable across a real thread
    /// boundary — the `reqwest_middleware::Middleware` bounds require
    /// `Send + Sync`. Moving a built layer into a spawned task on a
    /// multi-thread runtime fails to compile if the bound regresses; we assert
    /// on its real `Debug` output produced on the other thread.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_loadbalancer_layer_is_send_sync() {
        let layer =
            LoadbalancerLayerPoolMetrics::new(two_backend_config()).expect("valid config builds");
        let dbg = tokio::spawn(async move { format!("{layer:?}") })
            .await
            .expect("spawned task joins");
        assert!(dbg.contains("LoadbalancerLayerPoolMetrics"), "{dbg}");
    }
}
