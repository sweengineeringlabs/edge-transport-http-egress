//! Impl blocks for [`BreakerLayerBreakerMetrics`] — constructor +
//! [`reqwest_middleware::Middleware`] impl.

use std::sync::Arc;

use async_trait::async_trait;
use moka::future::Cache;
use tokio::sync::Mutex;

use edge_transport_breaker::DefaultBreakerTransition;
use edge_transport_breaker_policy::{
    Admission, AdmitRequest, BreakerNode, BreakerState, BreakerTransition, Outcome,
    RecordOutcomeRequest,
};

use crate::api::{
    BreakerConfig, BreakerError, BreakerLayerBreakerMetrics, FailureThresholdRequest,
    FailureThresholdResponse,
};

/// Capacity of the per-host breaker cache. Upper bound on the
/// number of distinct hosts we track state for. Beyond this,
/// moka evicts least-recently-used entries — which means a
/// burst of one-shot requests to different hosts doesn't lead
/// to unbounded memory growth, but also means breaker state
/// for rarely-contacted hosts gets forgotten (acceptable —
/// the host will re-trip if still unhealthy).
const MAX_TRACKED_HOSTS: u64 = 10_000;

impl BreakerLayerBreakerMetrics {
    /// Construct from a resolved config.
    pub(crate) fn new(config: BreakerConfig) -> Self {
        let cache: Cache<String, Arc<Mutex<BreakerNode>>> =
            Cache::builder().max_capacity(MAX_TRACKED_HOSTS).build();
        Self {
            config: Arc::new(config),
            state: cache,
        }
    }

    /// Get-or-insert per-host state.
    async fn host_state(&self, key: &str) -> Arc<Mutex<BreakerNode>> {
        self.state
            .get_with(key.to_string(), async {
                Arc::new(Mutex::new(BreakerNode {
                    state: BreakerState::Closed,
                    consecutive_failures: 0,
                    consecutive_successes: 0,
                }))
            })
            .await
    }

    /// Is this response status a "failure" for breaker-counting
    /// purposes?
    fn is_failure(&self, status: reqwest::StatusCode) -> bool {
        self.config.failure_statuses.contains(&status.as_u16())
    }
}

#[async_trait]
impl reqwest_middleware::Middleware for BreakerLayerBreakerMetrics {
    #[expect(
        clippy::expect_used,
        reason = "HostBreaker's CircuitBreakerNode/HostBreaker impls never return Err — the \
                  Result shape is a structural api/ contract requirement, not a real failure mode"
    )]
    async fn handle(
        &self,
        req: reqwest::Request,
        ext: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        // Key on authority (host:port) so the breaker's
        // granularity matches the consumer's intuition of "this
        // upstream is down."
        let key = match req.url().host_str() {
            Some(host) => match req.url().port() {
                Some(port) => format!("{host}:{port}"),
                None => host.to_string(),
            },
            // Hostless URL — treat as its own bucket.
            None => "__hostless__".to_string(),
        };

        let node = self.host_state(&key).await;

        // Admission decision under the lock.
        let admission = {
            let mut n = node.lock().await;
            let resp = DefaultBreakerTransition
                .admit(AdmitRequest {
                    state: n.state,
                    consecutive_failures: n.consecutive_failures,
                    consecutive_successes: n.consecutive_successes,
                    config: (*self.config).clone().into(),
                })
                .expect("BreakerTransition::admit is infallible");
            n.state = resp.state;
            n.consecutive_failures = resp.consecutive_failures;
            n.consecutive_successes = resp.consecutive_successes;
            resp.admission
        };

        match admission {
            Admission::RejectOpen => Err(reqwest_middleware::Error::Middleware(
                anyhow::Error::new(BreakerError::CircuitOpen { host: key }),
            )),
            Admission::Proceed => {
                let result = next.run(req, ext).await;

                // Classify + record the outcome under the lock.
                let outcome = match &result {
                    Ok(resp) if self.is_failure(resp.status()) => Outcome::Failure,
                    Ok(_) => Outcome::Success,
                    Err(_) => Outcome::Failure,
                };
                {
                    let mut n = node.lock().await;
                    let resp = DefaultBreakerTransition
                        .record(RecordOutcomeRequest {
                            state: n.state,
                            consecutive_failures: n.consecutive_failures,
                            consecutive_successes: n.consecutive_successes,
                            config: (*self.config).clone().into(),
                            outcome,
                        })
                        .expect("BreakerTransition::record is infallible");
                    n.state = resp.state;
                    n.consecutive_failures = resp.consecutive_failures;
                    n.consecutive_successes = resp.consecutive_successes;
                }

                result
            }
        }
    }
}

impl crate::api::BreakerMetrics for BreakerLayerBreakerMetrics {
    fn failure_threshold(
        &self,
        _request: FailureThresholdRequest,
    ) -> Result<FailureThresholdResponse, BreakerError> {
        Ok(FailureThresholdResponse {
            value: self.config.failure_threshold,
        })
    }
}

impl std::fmt::Debug for BreakerLayerBreakerMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("BreakerLayerBreakerMetrics");
        d.field("failure_threshold", &self.config.failure_threshold)
            .field(
                "half_open_after_seconds",
                &self.config.half_open_after_seconds,
            )
            .field("reset_after_successes", &self.config.reset_after_successes);
        d.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> BreakerConfig {
        BreakerConfig::from_config(
            r#"
                failure_threshold = 3
                half_open_after_seconds = 1
                reset_after_successes = 2
                failure_statuses = [500, 502, 503, 504]
            "#,
        )
        .unwrap()
    }

    /// @covers: is_failure
    /// Verifies config is accessible after construction (used inside host_state to
    /// initialise HostBreaker). The config field is wired correctly.
    #[test]
    fn test_host_state_config_is_accessible_after_construction() {
        let cfg = test_config();
        let l = BreakerLayerBreakerMetrics::new(cfg);
        // If config weren't stored, failure_statuses.contains() would not work.
        assert!(l.is_failure(reqwest::StatusCode::INTERNAL_SERVER_ERROR));
    }

    /// @covers: is_failure
    #[test]
    fn test_is_failure_classifies_configured_statuses() {
        let l = BreakerLayerBreakerMetrics::new(test_config());
        assert!(l.is_failure(reqwest::StatusCode::INTERNAL_SERVER_ERROR));
        assert!(l.is_failure(reqwest::StatusCode::SERVICE_UNAVAILABLE));
        assert!(!l.is_failure(reqwest::StatusCode::OK));
        assert!(!l.is_failure(reqwest::StatusCode::BAD_REQUEST));
    }

    /// @covers: host_state
    #[tokio::test]
    async fn test_host_state_shared_across_calls_for_same_key() {
        let l = BreakerLayerBreakerMetrics::new(test_config());
        let a = l.host_state("example.test").await;
        let b = l.host_state("example.test").await;
        assert!(Arc::ptr_eq(&a, &b));
    }

    /// @covers: host_state
    #[tokio::test]
    async fn test_host_state_distinct_across_hosts() {
        let l = BreakerLayerBreakerMetrics::new(test_config());
        let a = l.host_state("example.test").await;
        let b = l.host_state("another.test").await;
        assert!(!Arc::ptr_eq(&a, &b));
    }
}
