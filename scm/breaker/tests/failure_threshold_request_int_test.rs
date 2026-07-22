//! Integration tests for `FailureThresholdRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, BreakerMetricsFactory, FailureThresholdRequest, HttpBreakerSvcProcessor,
};

/// @covers: failure_threshold
#[test]
fn test_failure_threshold_request_reads_configured_value_happy() {
    let config = BreakerConfig {
        failure_threshold: 6,
        ..BreakerConfig::default()
    };
    let layer = HttpBreakerSvcProcessor::build_breaker_layer(config).expect("build ok");
    let metrics = BreakerMetricsFactory::from_layer(layer);
    let resp = metrics
        .failure_threshold(FailureThresholdRequest)
        .expect("infallible");
    assert_eq!(resp.value, 6);
}

/// @covers: failure_threshold
#[test]
fn test_failure_threshold_request_is_reusable_across_calls_edge() {
    let layer =
        HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default()).expect("build ok");
    let metrics = BreakerMetricsFactory::from_layer(layer);
    let a = metrics
        .failure_threshold(FailureThresholdRequest)
        .expect("infallible");
    let b = metrics
        .failure_threshold(FailureThresholdRequest)
        .expect("infallible");
    assert_eq!(a.value, b.value);
}
