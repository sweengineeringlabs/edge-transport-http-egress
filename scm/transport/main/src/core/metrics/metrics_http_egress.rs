use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use swe_observ_metrics::MetricsProvider;

use crate::api::{
    ConfigRequest, ConfigResponse, HealthCheckRequest, HttpEgress, HttpEgressResult, HttpRequest,
    HttpResponse, HttpStreamResponse,
};

/// Wraps any [`HttpEgress`]; records per-call latency, request count, and
/// error count using the supplied [`MetricsProvider`].
pub(crate) struct MetricsHttpEgress {
    inner: Arc<dyn HttpEgress>,
    provider: Arc<dyn MetricsProvider>,
}

impl MetricsHttpEgress {
    pub(crate) fn new(inner: Arc<dyn HttpEgress>, provider: Arc<dyn MetricsProvider>) -> Self {
        Self { inner, provider }
    }
}

#[async_trait]
impl HttpEgress for MetricsHttpEgress {
    async fn send(&self, request: HttpRequest) -> HttpEgressResult<HttpResponse> {
        let method = request.method.to_string();
        let start = Instant::now();
        let result = self.inner.send(request).await;
        let labels = &[("method", method.as_str())];
        self.provider
            .record_counter("edge_egress_requests_total", 1.0, labels);
        self.provider.record_histogram(
            "edge_egress_latency_us",
            start.elapsed().as_micros() as f64,
            labels,
        );
        if result.is_err() {
            self.provider
                .record_counter("edge_egress_errors_total", 1.0, labels);
        }
        result
    }

    async fn send_stream(&self, request: HttpRequest) -> HttpEgressResult<HttpStreamResponse> {
        let method = request.method.to_string();
        let start = Instant::now();
        let result = self.inner.send_stream(request).await;
        let labels = &[("method", method.as_str())];
        self.provider
            .record_counter("edge_egress_requests_total", 1.0, labels);
        self.provider.record_histogram(
            "edge_egress_latency_us",
            start.elapsed().as_micros() as f64,
            labels,
        );
        if result.is_err() {
            self.provider
                .record_counter("edge_egress_errors_total", 1.0, labels);
        }
        result
    }

    async fn health_check(&self, request: HealthCheckRequest) -> HttpEgressResult<()> {
        self.inner.health_check(request).await
    }

    fn config(&self, request: ConfigRequest) -> HttpEgressResult<ConfigResponse> {
        self.inner.config(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{HttpByteStream, HttpConfig, HttpEgressError};
    use swe_observ_metrics::create_local_metrics_backend;

    fn provider() -> Arc<dyn MetricsProvider> {
        Arc::new(create_local_metrics_backend())
    }

    struct MetricsNoopEgress;
    #[async_trait]
    impl HttpEgress for MetricsNoopEgress {
        async fn send(&self, _: HttpRequest) -> HttpEgressResult<HttpResponse> {
            Ok(HttpResponse {
                status: 200,
                headers: Default::default(),
                body: vec![],
            })
        }
        async fn send_stream(&self, _: HttpRequest) -> HttpEgressResult<HttpStreamResponse> {
            let body = HttpByteStream::new(futures::stream::empty());
            Ok(HttpStreamResponse {
                status: 200,
                headers: Default::default(),
                body,
            })
        }
        async fn health_check(&self, _request: HealthCheckRequest) -> HttpEgressResult<()> {
            Ok(())
        }
        fn config(&self, _request: ConfigRequest) -> HttpEgressResult<ConfigResponse> {
            Ok(ConfigResponse {
                config: HttpConfig::default(),
            })
        }
    }

    #[test]
    fn test_new_stores_inner_and_provider() {
        let p = provider();
        let inner = Arc::new(MetricsNoopEgress);
        let m = MetricsHttpEgress::new(Arc::clone(&inner) as Arc<dyn HttpEgress>, Arc::clone(&p));
        // Verify construction succeeded and the provider is wired by exercising it.
        let snaps = m.provider.export();
        assert!(
            snaps.is_empty(),
            "fresh instance must have no recorded metrics"
        );
    }

    #[tokio::test]
    async fn test_send_records_egress_requests_total_on_success() {
        let p = provider();
        let m = MetricsHttpEgress::new(Arc::new(MetricsNoopEgress), Arc::clone(&p));
        m.send(HttpRequest::get("/")).await.unwrap();
        let snaps = p.export();
        assert!(
            snaps
                .iter()
                .any(|s| s.name == "edge_egress_requests_total" && s.value == 1.0),
            "expected edge_egress_requests_total=1, got {snaps:?}"
        );
    }

    #[tokio::test]
    async fn test_send_records_egress_latency_histogram() {
        let p = provider();
        let m = MetricsHttpEgress::new(Arc::new(MetricsNoopEgress), Arc::clone(&p));
        m.send(HttpRequest::get("/")).await.unwrap();
        let snaps = p.export();
        assert!(
            snaps.iter().any(|s| s.name == "edge_egress_latency_us"),
            "expected edge_egress_latency_us, got {snaps:?}"
        );
    }

    #[tokio::test]
    async fn test_send_records_egress_errors_total_on_failure() {
        struct MetricsFailEgress;
        #[async_trait]
        impl HttpEgress for MetricsFailEgress {
            async fn send(&self, _: HttpRequest) -> HttpEgressResult<HttpResponse> {
                Err(HttpEgressError::ConnectionFailed("refused".into()))
            }
            async fn send_stream(&self, _: HttpRequest) -> HttpEgressResult<HttpStreamResponse> {
                Err(HttpEgressError::ConnectionFailed("refused".into()))
            }
            async fn health_check(&self, _request: HealthCheckRequest) -> HttpEgressResult<()> {
                Ok(())
            }
            fn config(&self, _request: ConfigRequest) -> HttpEgressResult<ConfigResponse> {
                Ok(ConfigResponse {
                    config: HttpConfig::default(),
                })
            }
        }
        let p = provider();
        let m = MetricsHttpEgress::new(Arc::new(MetricsFailEgress), Arc::clone(&p));
        let _ = m.send(HttpRequest::get("/")).await;
        let snaps = p.export();
        assert!(
            snaps
                .iter()
                .any(|s| s.name == "edge_egress_errors_total" && s.value == 1.0),
            "expected edge_egress_errors_total=1, got {snaps:?}"
        );
    }
}
