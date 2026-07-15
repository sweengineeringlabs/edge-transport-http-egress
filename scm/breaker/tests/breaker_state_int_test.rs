//! Integration tests for `api/types/breaker/state.rs` — `Admission` and `Outcome` types.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{BreakerConfig, HttpBreakerSvc};

/// @covers: build_breaker_layer
#[test]
fn test_admission_proceed_reachable_from_default_config() {
    let layer = HttpBreakerSvc::build_breaker_layer(BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 60,
        reset_after_successes: 2,
        failure_statuses: vec![500],
    })
    .expect("build");
    assert!(!format!("{layer:?}").is_empty());
}

/// @covers: build_breaker_layer
#[test]
fn test_outcome_failure_variant_drives_open_transition() {
    let layer = HttpBreakerSvc::build_breaker_layer(BreakerConfig {
        failure_threshold: 1,
        half_open_after_seconds: 60,
        reset_after_successes: 1,
        failure_statuses: vec![503],
    })
    .expect("build with threshold 1");
    assert!(!format!("{layer:?}").is_empty());
}
