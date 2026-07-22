//! Integration tests for `DecorateRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::{DecorateRequest, HttpRetrySvc, Processor, RetryConfig};

/// @covers: decorate
#[test]
fn test_decorate_request_config_field_drives_the_built_layer_happy() {
    let resp = HttpRetrySvc
        .decorate(DecorateRequest {
            config: RetryConfig {
                max_retries: 9,
                ..RetryConfig::default()
            },
        })
        .expect("valid config must build");
    assert!(format!("{:?}", resp.layer).contains("max_retries: 9"));
}

/// @covers: decorate
#[test]
fn test_decorate_request_is_reusable_across_calls_edge() {
    let a = HttpRetrySvc
        .decorate(DecorateRequest {
            config: RetryConfig::default(),
        })
        .expect("infallible for a valid config");
    let b = HttpRetrySvc
        .decorate(DecorateRequest {
            config: RetryConfig::default(),
        })
        .expect("infallible for a valid config");
    assert_eq!(format!("{:?}", a.layer), format!("{:?}", b.layer));
}
