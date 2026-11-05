//! Integration tests for `FailureThresholdResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker_api::{
    BreakerConfig, BreakerFactory, FailureThresholdRequest,
};
use edge_transport_http_egress_breaker_std::StdBreakerFactory;

/// @covers: FailureThresholdResponse
#[test]
fn test_failure_threshold_response_value_matches_default_config_happy() {
    let layer = StdBreakerFactory
        .build_breaker_layer(BreakerConfig::default())
        .expect("build ok");
    let metrics = layer;
    let resp = metrics
        .failure_threshold(FailureThresholdRequest)
        .expect("infallible");
    assert_eq!(resp.value, BreakerConfig::default().failure_threshold);
}

/// @covers: FailureThresholdResponse
#[test]
fn test_failure_threshold_response_value_is_nonzero_edge() {
    let layer = StdBreakerFactory
        .build_breaker_layer(BreakerConfig::default())
        .expect("build ok");
    let metrics = layer;
    let resp = metrics
        .failure_threshold(FailureThresholdRequest)
        .expect("infallible");
    assert_ne!(resp.value, 0);
}
