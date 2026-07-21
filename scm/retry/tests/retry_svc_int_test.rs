//! Anchor tests for `saf/retry/retry_svc.rs` — SEA Rule 220 compliance.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::{DecorateRequest, HttpRetrySvc, Processor, RetryConfig};

#[test]
fn test_create_config_builder_returns_named_builder_happy() {
    let b = HttpRetrySvc::new_app_config_builder();
    assert!(!b.name().is_empty());
}

#[test]
fn test_build_retry_layer_default_config_succeeds_happy() {
    let layer = HttpRetrySvc
        .decorate(DecorateRequest {
            config: RetryConfig::default(),
        })
        .expect("default config must build")
        .layer;
    // Payload assertion: default max_retries=3 must be embedded.
    assert!(
        format!("{layer:?}").contains("max_retries: 3"),
        "built layer must carry the default max_retries=3"
    );
    // Sibling negative: an invalid config is rejected by validate().
    let bad = RetryConfig {
        multiplier: 0.0,
        ..RetryConfig::default()
    };
    assert!(bad.validate().is_err(), "multiplier=0 must fail validate()");
}

#[test]
fn test_build_retry_layer_called_twice_both_succeed_edge() {
    let r1 = HttpRetrySvc.decorate(DecorateRequest {
        config: RetryConfig::default(),
    });
    let r2 = HttpRetrySvc.decorate(DecorateRequest {
        config: RetryConfig::default(),
    });
    assert!(r1.is_ok() && r2.is_ok());
}
