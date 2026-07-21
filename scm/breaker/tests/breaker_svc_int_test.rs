//! Integration tests for `HttpBreakerSvcProcessor::build_breaker_layer`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, BreakerMetrics, FailureThresholdRequest, HttpBreakerSvcProcessor,
};

/// @covers: build_breaker_layer
#[test]
fn test_build_breaker_layer_with_default_config_succeeds() {
    let layer = HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default())
        .expect("build_breaker_layer with default config must succeed");
    let resp = layer
        .failure_threshold(FailureThresholdRequest)
        .expect("failure_threshold is infallible");
    assert_eq!(
        resp.value, 5,
        "layer must carry the default config's failure_threshold, not a stub value"
    );
}
