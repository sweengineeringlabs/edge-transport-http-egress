//! Integration tests for `get_failure_threshold` — `BreakerMetrics` contract via SAF wrapper.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{get_failure_threshold, BreakerConfig, HttpBreakerSvc};

/// @covers: get_failure_threshold
#[test]
fn breaker_trait_breaker_metrics_get_failure_threshold_returns_configured_value_int_test() {
    let config = BreakerConfig {
        failure_threshold: 7,
        half_open_after_seconds: 10,
        reset_after_successes: 2,
        failure_statuses: vec![500],
    };
    let layer = HttpBreakerSvc::build_breaker_layer(config).expect("build must succeed");
    assert_eq!(get_failure_threshold(&layer), 7);
}

/// @covers: HttpBreakerSvc::create_config_builder — dep coverage for swe-edge-configbuilder
#[test]
fn breaker_struct_http_breaker_svc_create_config_builder_returns_seeded_builder_int_test() {
    let builder = HttpBreakerSvc::create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must be seeded with crate name"
    );
}
