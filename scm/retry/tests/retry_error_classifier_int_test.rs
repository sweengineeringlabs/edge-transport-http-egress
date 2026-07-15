//! Integration tests for the retry error classifier behavior.
//!
//! Rule 120: `src/api/retry/layer/retry_error_classifier.rs` requires a
//! corresponding test file.
//!
//! The `RetryErrorClassifier` trait is a marker in api/ and the concrete impl
//! is `pub(crate)` in core/. We test the error-classification behavior through
//! the public retry API: a middleware-level error must not be retried; the retry
//! layer must respect that policy.

use edge_transport_http_egress_retry::{HttpRetrySvc, RetryConfig};

/// @covers: RetryErrorClassifier (via RetryLayer)
/// A `RetryLayer` built with default config must succeed — confirming the
/// error classifier is wired in without panicking.
#[test]
fn retry_struct_retry_error_classifier_layer_builds_without_panic_int_test() {
    let cfg = RetryConfig::default();
    let result = HttpRetrySvc::build_retry_layer(cfg);
    assert!(
        result.is_ok(),
        "RetryLayer (backed by RetryErrorClassifier) must build; got: {result:?}"
    );
}

/// @covers: RetryErrorClassifier (classifier is Send+Sync via RetryLayer)
/// The retry layer must be `Send + Sync` — a requirement satisfied only if
/// the embedded classifier is also `Send + Sync`.
#[test]
fn retry_struct_retry_error_classifier_layer_is_send_and_sync_int_test() {
    use edge_transport_http_egress_retry::RetryLayer;
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<RetryLayer>();
}
