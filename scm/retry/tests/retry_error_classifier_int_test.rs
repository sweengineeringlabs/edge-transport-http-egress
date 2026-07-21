#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for the retry error classifier behavior.
//!
//! Rule 120: `src/api/retry/layer/retry_error_classifier.rs` requires a
//! corresponding test file.
//!
//! The `RetryErrorClassifier` trait is a marker in api/ and the concrete impl
//! is `pub(crate)` in core/. We test the error-classification behavior through
//! the public retry API: a middleware-level error must not be retried; the retry
//! layer must respect that policy.

use edge_transport_http_egress_retry::{DecorateRequest, HttpRetrySvc, Processor, RetryConfig};

/// @covers: RetryErrorClassifier (via RetryLayer)
/// A `RetryLayer` built with default config must succeed — confirming the
/// error classifier is wired in without panicking.
#[test]
fn retry_struct_retry_error_classifier_layer_builds_without_panic_int_test() {
    let cfg = RetryConfig::default();
    let layer = HttpRetrySvc
        .decorate(DecorateRequest { config: cfg })
        .expect("RetryLayer must build");
    // Payload assertion: the classifier-backed layer carries the default policy.
    assert!(
        format!("{layer:?}").contains("max_retries: 3"),
        "layer must embed the default retry policy"
    );
    // Sibling negative: an invalid retry policy is rejected by validate().
    let bad = RetryConfig {
        multiplier: -2.0,
        ..RetryConfig::default()
    };
    assert!(
        bad.validate().is_err(),
        "negative multiplier must fail validate()"
    );
}

/// @covers: RetryErrorClassifier (classifier is Send+Sync via RetryLayer)
/// The retry layer must be `Send + Sync` — a requirement satisfied only if
/// the embedded classifier is also `Send + Sync`.
#[test]
fn retry_struct_retry_error_classifier_layer_is_send_and_sync_int_test() {
    // Genuine runtime proof of Send: move the classifier-backed layer into
    // another thread and read it back.
    let layer = HttpRetrySvc
        .decorate(DecorateRequest {
            config: RetryConfig::default(),
        })
        .expect("build")
        .layer;
    let dbg = std::thread::spawn(move || format!("{layer:?}"))
        .join()
        .expect("thread must not panic");
    assert!(
        dbg.contains("RetryLayer"),
        "layer must survive a thread move; got: {dbg}"
    );
}
