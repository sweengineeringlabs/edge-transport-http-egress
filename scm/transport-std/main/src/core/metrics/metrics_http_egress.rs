use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use edge_transport_http_egress_transport_api::{
    HttpMetricKind, HttpMetricObservation, HttpMetricsObserver,
};

use edge_transport_http_egress_transport_api::{
    ConfigRequest, ConfigResponse, HealthCheckRequest, HttpEgress, HttpEgressResult, HttpRequest,
    HttpResponse, HttpSecurityContext, HttpStreamResponse,
};

/// Wraps any [`HttpEgress`]; records per-call latency, request count, and
/// error count using the supplied [`HttpMetricsObserver`].
pub(crate) struct MetricsHttpEgress {
    inner: Arc<dyn HttpEgress>,
    observer: HttpMetricsObserver,
}

impl MetricsHttpEgress {
    pub(crate) fn new(inner: Arc<dyn HttpEgress>, observer: HttpMetricsObserver) -> Self {
        Self { inner, observer }
    }
}

#[async_trait]
impl HttpEgress for MetricsHttpEgress {
    async fn send_with_context(
        &self,
        request: HttpRequest,
        ctx: HttpSecurityContext,
    ) -> HttpEgressResult<HttpResponse> {
        let method = request.method.to_string();
        let start = Instant::now();
        let result = self.inner.send_with_context(request, ctx).await;
        let labels = &[("method", method.as_str())];
        (self.observer)(HttpMetricObservation {
            name: "edge_egress_requests_total",
            kind: HttpMetricKind::Counter,
            value: 1.0,
            labels,
        });
        (self.observer)(HttpMetricObservation {
            name: "edge_egress_latency_us",
            kind: HttpMetricKind::Histogram,
            value: start.elapsed().as_micros() as f64,
            labels,
        });
        if result.is_err() {
            (self.observer)(HttpMetricObservation {
                name: "edge_egress_errors_total",
                kind: HttpMetricKind::Counter,
                value: 1.0,
                labels,
            });
        }
        result
    }

    async fn send_stream(&self, request: HttpRequest) -> HttpEgressResult<HttpStreamResponse> {
        let method = request.method.to_string();
        let start = Instant::now();
        let result = self.inner.send_stream(request).await;
        let labels = &[("method", method.as_str())];
        (self.observer)(HttpMetricObservation {
            name: "edge_egress_requests_total",
            kind: HttpMetricKind::Counter,
            value: 1.0,
            labels,
        });
        (self.observer)(HttpMetricObservation {
            name: "edge_egress_latency_us",
            kind: HttpMetricKind::Histogram,
            value: start.elapsed().as_micros() as f64,
            labels,
        });
        if result.is_err() {
            (self.observer)(HttpMetricObservation {
                name: "edge_egress_errors_total",
                kind: HttpMetricKind::Counter,
                value: 1.0,
                labels,
            });
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
    use edge_transport_http_egress_transport_api::{HttpByteStream, HttpConfig, HttpEgressError};
    use futures::StreamExt;
    use std::sync::Mutex;

    #[derive(Debug)]
    struct CapturedMetric {
        name: &'static str,
        kind: HttpMetricKind,
        value: f64,
        labels: Vec<(String, String)>,
    }
    type Events = Arc<Mutex<Vec<CapturedMetric>>>;
    fn observer() -> (HttpMetricsObserver, Events) {
        let events: Events = Arc::new(Mutex::new(Vec::new()));
        let captured = events.clone();
        let observer: HttpMetricsObserver = Arc::new(move |event| {
            captured.lock().unwrap().push(CapturedMetric {
                name: event.name,
                kind: event.kind,
                value: event.value,
                labels: event
                    .labels
                    .iter()
                    .map(|(key, value)| (key.to_string(), value.to_string()))
                    .collect(),
            });
        });
        (observer, events)
    }

    struct StubEgress {
        status: u16,
        failed: bool,
        body_fails: bool,
    }
    impl Default for StubEgress {
        fn default() -> Self {
            Self {
                status: 200,
                failed: false,
                body_fails: false,
            }
        }
    }
    #[async_trait]
    impl HttpEgress for StubEgress {
        async fn send_with_context(
            &self,
            _: HttpRequest,
            _: HttpSecurityContext,
        ) -> HttpEgressResult<HttpResponse> {
            if self.failed {
                return Err(HttpEgressError::ConnectionFailed("refused".into()));
            }
            Ok(HttpResponse {
                status: self.status,
                headers: Default::default(),
                body: Vec::new(),
            })
        }
        async fn send_stream(&self, _: HttpRequest) -> HttpEgressResult<HttpStreamResponse> {
            if self.failed {
                return Err(HttpEgressError::ConnectionFailed("refused".into()));
            }
            let body = if self.body_fails {
                HttpByteStream::new(futures::stream::iter([Err::<Vec<u8>, _>(
                    HttpEgressError::ConnectionFailed("body failed".into()),
                )]))
            } else {
                HttpByteStream::new(futures::stream::empty())
            };
            Ok(HttpStreamResponse {
                status: self.status,
                headers: Default::default(),
                body,
            })
        }
        async fn health_check(&self, _: HealthCheckRequest) -> HttpEgressResult<()> {
            Ok(())
        }
        fn config(&self, _: ConfigRequest) -> HttpEgressResult<ConfigResponse> {
            Ok(ConfigResponse {
                config: HttpConfig::default(),
            })
        }
    }

    #[test]
    fn test_new_stores_inner_and_provider() {
        let (observer, events) = observer();
        let wrapper = MetricsHttpEgress::new(Arc::new(StubEgress::default()), observer);
        assert!(events.lock().unwrap().is_empty());
        (wrapper.observer)(HttpMetricObservation {
            name: "wiring",
            kind: HttpMetricKind::Counter,
            value: 2.0,
            labels: &[],
        });
        assert_eq!(events.lock().unwrap()[0].name, "wiring");
    }
    #[tokio::test]
    async fn test_send_records_egress_requests_total_on_success() {
        let (observer, events) = observer();
        let wrapper = MetricsHttpEgress::new(Arc::new(StubEgress::default()), observer);
        assert_eq!(
            wrapper.send(HttpRequest::get("/")).await.unwrap().status,
            200
        );
        let captured = events.lock().unwrap();
        let counter = captured
            .iter()
            .find(|metric| metric.name == "edge_egress_requests_total")
            .unwrap();
        assert_eq!(counter.value, 1.0);
        assert_eq!(counter.kind, HttpMetricKind::Counter);
        assert_eq!(counter.labels, [("method".to_string(), "GET".to_string())]);
        assert_eq!(captured.len(), 2);
    }
    #[tokio::test]
    async fn test_send_records_egress_latency_histogram() {
        let (observer, events) = observer();
        let wrapper = MetricsHttpEgress::new(Arc::new(StubEgress::default()), observer);
        wrapper.send(HttpRequest::get("/")).await.unwrap();
        let captured = events.lock().unwrap();
        let histogram = captured
            .iter()
            .find(|metric| metric.name == "edge_egress_latency_us")
            .unwrap();
        assert_eq!(histogram.kind, HttpMetricKind::Histogram);
        assert!(histogram.value >= 0.0);
        assert_eq!(
            histogram.labels,
            [("method".to_string(), "GET".to_string())]
        );
    }
    #[tokio::test]
    async fn test_send_records_egress_errors_total_on_failure() {
        let (observer, events) = observer();
        let wrapper = MetricsHttpEgress::new(
            Arc::new(StubEgress {
                failed: true,
                ..Default::default()
            }),
            observer,
        );
        assert!(
            matches!(wrapper.send(HttpRequest::get("/")).await, Err(HttpEgressError::ConnectionFailed(message)) if message == "refused")
        );
        let captured = events.lock().unwrap();
        assert_eq!(captured.len(), 3);
        assert_eq!(captured[2].name, "edge_egress_errors_total");
        assert_eq!(captured[2].value, 1.0);
        assert_eq!(captured[2].kind, HttpMetricKind::Counter);
    }
    #[tokio::test]
    async fn test_http_status_503_retains_successful_transport_result() {
        let (observer, events) = observer();
        let wrapper = MetricsHttpEgress::new(
            Arc::new(StubEgress {
                status: 503,
                ..Default::default()
            }),
            observer,
        );
        assert_eq!(
            wrapper.send(HttpRequest::get("/")).await.unwrap().status,
            503
        );
        assert_eq!(
            events.lock().unwrap().len(),
            2,
            "status responses do not count as transport errors"
        );
    }
    #[tokio::test]
    async fn test_stream_open_emits_measurements_before_body_consumption() {
        let (observer, events) = observer();
        let wrapper = MetricsHttpEgress::new(Arc::new(StubEgress::default()), observer);
        let mut response = wrapper.send_stream(HttpRequest::get("/")).await.unwrap();
        assert_eq!(events.lock().unwrap().len(), 2);
        assert!(response.body.next().await.is_none());
        assert_eq!(events.lock().unwrap().len(), 2);
    }
    #[tokio::test]
    async fn test_stream_open_failure_records_error_without_changing_failure() {
        let (observer, events) = observer();
        let wrapper = MetricsHttpEgress::new(
            Arc::new(StubEgress {
                failed: true,
                ..Default::default()
            }),
            observer,
        );
        assert!(
            matches!(wrapper.send_stream(HttpRequest::get("/")).await, Err(HttpEgressError::ConnectionFailed(message)) if message == "refused")
        );
        assert_eq!(events.lock().unwrap().len(), 3);
        assert_eq!(events.lock().unwrap()[2].name, "edge_egress_errors_total");
    }
    #[tokio::test]
    async fn test_late_stream_body_failure_does_not_add_open_failure_metric() {
        let (observer, events) = observer();
        let wrapper = MetricsHttpEgress::new(
            Arc::new(StubEgress {
                body_fails: true,
                ..Default::default()
            }),
            observer,
        );
        let mut response = wrapper.send_stream(HttpRequest::get("/")).await.unwrap();
        assert!(
            matches!(response.body.next().await, Some(Err(HttpEgressError::ConnectionFailed(message))) if message == "body failed")
        );
        assert_eq!(events.lock().unwrap().len(), 2);
    }
    #[tokio::test]
    async fn test_health_and_config_do_not_emit_measurements() {
        let (observer, events) = observer();
        let wrapper = MetricsHttpEgress::new(Arc::new(StubEgress::default()), observer);
        wrapper.health_check(HealthCheckRequest).await.unwrap();
        wrapper.config(ConfigRequest).unwrap();
        assert!(events.lock().unwrap().is_empty());
    }
    #[tokio::test]
    async fn test_shared_observer_retains_all_concurrent_measurements() {
        let (observer, events) = observer();
        let wrapper = Arc::new(MetricsHttpEgress::new(
            Arc::new(StubEgress::default()),
            observer,
        ));
        let mut tasks = Vec::new();
        for _ in 0..100 {
            let wrapper = wrapper.clone();
            tasks.push(tokio::spawn(async move {
                wrapper.send(HttpRequest::get("/")).await
            }));
        }
        for task in tasks {
            task.await.unwrap().unwrap();
        }
        let captured = events.lock().unwrap();
        assert_eq!(
            captured
                .iter()
                .filter(|event| event.name == "edge_egress_requests_total")
                .count(),
            100
        );
        assert_eq!(
            captured
                .iter()
                .filter(|event| event.kind == HttpMetricKind::Histogram)
                .count(),
            100
        );
    }
}
