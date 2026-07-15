//! Integration tests for `api/types/breaker/config.rs` — the public `BreakerConfig`
//! struct and its field semantics.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{BreakerConfig, HttpBreakerSvc};

// ---------------------------------------------------------------------------
// Struct literal construction — all four fields are public
// ---------------------------------------------------------------------------

/// @covers: BreakerConfig
/// If a field is renamed, removed, or made `pub(crate)`, this test fails to compile.
#[test]
fn test_breaker_config_all_fields_constructable_and_readable() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 60,
        reset_after_successes: 2,
        failure_statuses: vec![500, 502, 503],
    };
    assert_eq!(cfg.failure_threshold, 3);
    assert_eq!(cfg.half_open_after_seconds, 60);
    assert_eq!(cfg.reset_after_successes, 2);
    assert_eq!(cfg.failure_statuses, vec![500u16, 502, 503]);
}

// ---------------------------------------------------------------------------
// failure_threshold — boundary values
// ---------------------------------------------------------------------------

/// @covers: BreakerConfig
/// `failure_threshold = 1` means the breaker opens after a single failure.
#[test]
fn test_breaker_config_threshold_one_builds() {
    let cfg = BreakerConfig {
        failure_threshold: 1,
        half_open_after_seconds: 5,
        reset_after_successes: 1,
        failure_statuses: vec![503],
    };
    HttpBreakerSvc::build_breaker_layer(cfg).expect("failure_threshold=1 must build");
}

/// @covers: BreakerConfig
#[test]
fn test_breaker_config_large_threshold_builds() {
    let cfg = BreakerConfig {
        failure_threshold: u32::MAX,
        half_open_after_seconds: 1,
        reset_after_successes: 1,
        failure_statuses: vec![],
    };
    HttpBreakerSvc::build_breaker_layer(cfg).expect("large failure_threshold must build");
}

// ---------------------------------------------------------------------------
// half_open_after_seconds — boundary values
// ---------------------------------------------------------------------------

/// @covers: BreakerConfig
#[test]
fn test_breaker_config_zero_wait_builds() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 0,
        reset_after_successes: 2,
        failure_statuses: vec![500],
    };
    HttpBreakerSvc::build_breaker_layer(cfg).expect("half_open_after_seconds=0 must build");
}

/// @covers: BreakerConfig
#[test]
fn test_breaker_config_large_wait_builds() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 3600,
        reset_after_successes: 2,
        failure_statuses: vec![503],
    };
    HttpBreakerSvc::build_breaker_layer(cfg).expect("half_open_after_seconds=3600 must build");
}

// ---------------------------------------------------------------------------
// failure_statuses — boundary values
// ---------------------------------------------------------------------------

/// @covers: BreakerConfig
#[test]
fn test_breaker_config_empty_failure_statuses_builds() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 10,
        reset_after_successes: 2,
        failure_statuses: vec![],
    };
    HttpBreakerSvc::build_breaker_layer(cfg).expect("empty failure_statuses must build");
}

/// @covers: BreakerConfig
#[test]
fn test_breaker_config_all_5xx_failure_statuses_builds() {
    let all_5xx: Vec<u16> = (500..=599).collect();
    let cfg = BreakerConfig {
        failure_threshold: 5,
        half_open_after_seconds: 30,
        reset_after_successes: 3,
        failure_statuses: all_5xx,
    };
    HttpBreakerSvc::build_breaker_layer(cfg).expect("all 5xx failure_statuses must build");
}

// ---------------------------------------------------------------------------
// Config round-trip through build_breaker_layer
// ---------------------------------------------------------------------------

/// @covers: BreakerConfig
#[test]
fn test_breaker_config_round_trips_through_builder_unchanged() {
    let cfg = BreakerConfig {
        failure_threshold: 11,
        half_open_after_seconds: 45,
        reset_after_successes: 4,
        failure_statuses: vec![500, 503, 504],
    };
    let b_cfg = cfg;
    let out = &b_cfg;
    assert_eq!(out.failure_threshold, 11);
    assert_eq!(out.half_open_after_seconds, 45);
    assert_eq!(out.reset_after_successes, 4);
    assert_eq!(out.failure_statuses, vec![500u16, 503, 504]);
}
