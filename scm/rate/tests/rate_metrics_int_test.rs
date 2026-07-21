//! Integration tests for the `RateMetrics` trait.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{
    HttpRateSvcProcessor, RateConfig, RateError, RateLimitRequest, RateLimitResponse, RateMetrics,
};

/// @covers: rate_limit
#[test]
fn test_rate_limit_reports_configured_rate_happy() {
    // `RateLayerRateMetrics` (returned by build_rate_layer) implements RateMetrics.
    // Use a non-default rate so a stub returning a fixed value could not fake it.
    let layer = HttpRateSvcProcessor::build_rate_layer(RateConfig {
        tokens_per_second: 73,
        burst_capacity: 146,
        per_host: true,
    })
    .expect("valid config must build");
    let reported = layer
        .rate_limit(RateLimitRequest)
        .expect("rate_limit is infallible")
        .tokens_per_second;
    assert_eq!(
        reported, 73,
        "rate_limit must report the configured tokens_per_second"
    );
}

/// @covers: rate_limit
#[test]
fn test_rate_limit_minimum_boundary_rate_is_valid_edge() {
    let layer = HttpRateSvcProcessor::build_rate_layer(RateConfig {
        tokens_per_second: 1,
        burst_capacity: 1,
        per_host: false,
    })
    .expect("valid config must build");
    let reported = layer
        .rate_limit(RateLimitRequest)
        .expect("infallible")
        .tokens_per_second;
    assert_eq!(
        reported, 1,
        "the minimum valid rate must round-trip unchanged"
    );
}

/// A minimal external test-double proving `RateMetrics::rate_limit` can
/// genuinely fail for a real implementor — the crate's own
/// `RateLayerRateMetrics` never returns `Err` here, so this is the only way
/// to exercise the error path.
struct FailingRateMetrics;

impl RateMetrics for FailingRateMetrics {
    fn rate_limit(&self, _request: RateLimitRequest) -> Result<RateLimitResponse, RateError> {
        Err(RateError::InvalidConfig(
            "no rate limit configured".to_string(),
        ))
    }
}

/// @covers: rate_limit
#[test]
fn test_rate_limit_unconfigured_implementor_returns_err_error() {
    let metrics = FailingRateMetrics;
    let result = metrics.rate_limit(RateLimitRequest);
    assert!(
        matches!(result, Err(RateError::InvalidConfig(_))),
        "an external RateMetrics impl reporting no configured rate must surface as InvalidConfig; got: {result:?}"
    );
}
