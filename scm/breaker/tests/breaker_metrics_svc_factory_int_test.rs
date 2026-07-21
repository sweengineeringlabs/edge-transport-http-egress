//! Integration tests for [`BreakerMetricsFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, BreakerMetricsFactory, FailureThresholdRequest, HttpBreakerSvcProcessor,
};

/// @covers: from_layer
#[test]
fn test_from_layer_upcasts_to_metrics_happy() {
    let config = BreakerConfig {
        failure_threshold: 9,
        half_open_after_seconds: 5,
        reset_after_successes: 2,
        failure_statuses: vec![500],
    };
    let layer = HttpBreakerSvcProcessor::build_breaker_layer(config).expect("build ok");
    let metrics = BreakerMetricsFactory::from_layer(layer);
    let resp = metrics
        .failure_threshold(FailureThresholdRequest)
        .expect("upcast metrics must genuinely work");
    assert_eq!(resp.value, 9);
}

/// @covers: from_layer
#[test]
fn test_from_layer_reflects_a_non_default_config_value_edge() {
    // Boundary-adjacent scenario: prove the value reflects the caller's real
    // config rather than a hardcoded stub — a non-default, non-zero
    // threshold survives the upcast.
    let config = BreakerConfig {
        failure_threshold: 42,
        ..BreakerConfig::default()
    };
    let layer = HttpBreakerSvcProcessor::build_breaker_layer(config).expect("build ok");
    let metrics = BreakerMetricsFactory::from_layer(layer);
    let resp = metrics
        .failure_threshold(FailureThresholdRequest)
        .expect("must succeed");
    assert_ne!(resp.value, 0);
    assert_eq!(resp.value, 42);
}

/// @covers: from_layer
#[test]
fn test_from_layer_generic_over_pool_state_edge() {
    let layer =
        HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default()).expect("build ok");
    let metrics = BreakerMetricsFactory::from_layer(layer);
    let resp = metrics
        .failure_threshold(FailureThresholdRequest)
        .expect("must succeed");
    assert_eq!(resp.value, BreakerConfig::default().failure_threshold);
}
