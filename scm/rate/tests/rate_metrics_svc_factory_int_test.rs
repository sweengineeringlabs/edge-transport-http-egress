//! Integration tests for the SAF `RateMetricsFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{
    HttpRateSvcProcessor, RateConfig, RateLimitRequest, RateMetricsFactory,
};

/// @covers: RateMetricsFactory
#[test]
fn test_rate_metrics_factory_from_layer_exposes_rate_limit() {
    let layer = HttpRateSvcProcessor::build_rate_layer(RateConfig {
        tokens_per_second: 55,
        burst_capacity: 110,
        per_host: false,
    })
    .expect("valid config must build");
    let metrics = RateMetricsFactory::from_layer(layer);
    let reported = metrics
        .rate_limit(RateLimitRequest)
        .expect("rate_limit is infallible")
        .tokens_per_second;
    assert_eq!(
        reported, 55,
        "factory-exposed metrics must report the layer's configured rate"
    );
}
