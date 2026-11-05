//! Integration tests for the standard processor behind `StdRetryFactory::create_processor`:
//! turning a `RetryConfig` into an attachable retry layer.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry_api::{
    DecorateRequest, RetryConfig, RetryError, RetryFactory,
};
use edge_transport_http_egress_retry_std::StdRetryFactory;

fn config(max_retries: u32) -> RetryConfig {
    RetryConfig {
        max_retries,
        ..RetryConfig::default()
    }
}

/// @covers: decorate
#[test]
fn test_decorate_builds_a_layer_carrying_the_configured_retry_budget_happy() {
    let decorated = StdRetryFactory
        .create_processor()
        .decorate(DecorateRequest { config: config(4) })
        .expect("a valid config must decorate");
    let debug = format!("{:?}", decorated.layer);
    assert!(
        debug.contains("max_retries: 4"),
        "the layer must carry the configured budget, got: {debug}"
    );
}

/// @covers: decorate
#[test]
fn test_decorate_rejects_an_invalid_config_error() {
    let err = StdRetryFactory
        .create_processor()
        .decorate(DecorateRequest {
            config: RetryConfig {
                multiplier: 0.0,
                ..RetryConfig::default()
            },
        })
        .expect_err("a zero multiplier must be rejected before a layer is built");
    assert!(matches!(err, RetryError::InvalidConfig(_)), "got: {err:?}");
}

/// @covers: decorate
#[test]
fn test_decorate_called_twice_builds_independent_layers_edge() {
    let processor = StdRetryFactory.create_processor();
    let a = processor
        .decorate(DecorateRequest { config: config(1) })
        .unwrap();
    let b = processor
        .decorate(DecorateRequest { config: config(9) })
        .unwrap();
    assert!(format!("{:?}", a.layer).contains("max_retries: 1"));
    assert!(format!("{:?}", b.layer).contains("max_retries: 9"));
}
