#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Regression tests for retry behavior with middleware errors.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use edge_transport_http_egress_retry::{DecorateRequest, HttpRetrySvc, Processor, RetryConfig};

struct FailingMiddleware {
    calls: Arc<AtomicUsize>,
}

#[async_trait]
impl reqwest_middleware::Middleware for FailingMiddleware {
    async fn handle(
        &self,
        _req: reqwest::Request,
        _ext: &mut http::Extensions,
        _next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Err(reqwest_middleware::Error::middleware(
            std::io::Error::other("synthetic downstream middleware failure"),
        ))
    }
}

/// Middleware/configuration errors are not transient transport failures and
/// must not trigger another attempt, even for a retryable method.
#[tokio::test]
async fn middleware_error_is_returned_without_retrying() {
    let retry = HttpRetrySvc
        .decorate(DecorateRequest {
            config: RetryConfig {
                max_retries: 3,
                initial_interval_ms: 0,
                max_interval_ms: 0,
                retryable_methods: vec!["GET".into()],
                ..RetryConfig::default()
            },
        })
        .expect("valid retry config")
        .layer;
    let calls = Arc::new(AtomicUsize::new(0));
    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(retry)
        .with_arc(Arc::new(FailingMiddleware {
            calls: Arc::clone(&calls),
        }))
        .build();

    let error = client
        .get("http://example.invalid/")
        .send()
        .await
        .expect_err("downstream middleware returns an error");

    assert!(matches!(error, reqwest_middleware::Error::Middleware(_)));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
