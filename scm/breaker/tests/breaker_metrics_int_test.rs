//! Integration tests for `get_failure_threshold` — `BreakerMetrics` contract via SAF wrapper.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, BreakerMetrics, FailureThresholdRequest, HttpBreakerSvcProcessor,
};

/// @covers: BreakerMetrics::failure_threshold
#[test]
fn breaker_trait_breaker_metrics_failure_threshold_returns_configured_value_int_test() {
    let config = BreakerConfig {
        failure_threshold: 7,
        half_open_after_seconds: 10,
        reset_after_successes: 2,
        failure_statuses: vec![500],
    };
    let layer = HttpBreakerSvcProcessor::build_breaker_layer(config).expect("build must succeed");
    let threshold = layer
        .failure_threshold(FailureThresholdRequest)
        .expect("reading the configured threshold must succeed");
    assert_eq!(threshold.value, 7);
}
