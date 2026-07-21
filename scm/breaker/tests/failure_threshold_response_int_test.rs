//! Integration tests for `FailureThresholdResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, BreakerMetricsFactory, FailureThresholdRequest, HttpBreakerSvcProcessor,
};

/// @covers: FailureThresholdResponse
#[test]
fn test_failure_threshold_response_value_matches_default_config_happy() {
    let layer =
        HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default()).expect("build ok");
    let metrics = BreakerMetricsFactory::from_layer(layer);
    let resp = metrics
        .failure_threshold(FailureThresholdRequest)
        .expect("infallible");
    assert_eq!(resp.value, BreakerConfig::default().failure_threshold);
}

/// @covers: FailureThresholdResponse
#[test]
fn test_failure_threshold_response_value_is_nonzero_edge() {
    let layer =
        HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default()).expect("build ok");
    let metrics = BreakerMetricsFactory::from_layer(layer);
    let resp = metrics
        .failure_threshold(FailureThresholdRequest)
        .expect("infallible");
    assert_ne!(resp.value, 0);
}
