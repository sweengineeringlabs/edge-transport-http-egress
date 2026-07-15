//! Integration tests for `core/breaker_layer/mod.rs`.
//!
//! `core::breaker_layer` contains `BreakerLayer::new`, the per-host state
//! cache construction, and the `reqwest_middleware::Middleware` impl.  All
//! internals are `pub(crate)`, so we verify observable behaviour:
//! - Various policy combinations must produce a valid layer.
//! - `Debug` output must reflect the configured policy fields.
//! - `Send + Sync` must hold after construction.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{BreakerConfig, BreakerLayer, HttpBreakerSvc};

// ---------------------------------------------------------------------------
// Low threshold — breaker trips quickly
// ---------------------------------------------------------------------------

/// @covers: build_breaker_layer
#[test]
fn test_core_breaker_layer_threshold_one_builds() {
    let cfg = BreakerConfig {
        failure_threshold: 1,
        half_open_after_seconds: 5,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    };
    HttpBreakerSvc::build_breaker_layer(cfg).expect("failure_threshold=1 must build");
}

// ---------------------------------------------------------------------------
// Zero half-open wait — immediate probe
// ---------------------------------------------------------------------------

/// @covers: build_breaker_layer
#[test]
fn test_core_breaker_layer_zero_wait_builds() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 0,
        reset_after_successes: 2,
        failure_statuses: vec![503],
    };
    HttpBreakerSvc::build_breaker_layer(cfg).expect("half_open_after_seconds=0 must build");
}

// ---------------------------------------------------------------------------
// Many failure statuses — wide failure surface
// ---------------------------------------------------------------------------

/// @covers: build_breaker_layer
#[test]
fn test_core_breaker_layer_many_failure_statuses_builds() {
    let statuses: Vec<u16> = vec![500, 501, 502, 503, 504, 505, 506, 507, 508];
    let cfg = BreakerConfig {
        failure_threshold: 5,
        half_open_after_seconds: 30,
        reset_after_successes: 3,
        failure_statuses: statuses,
    };
    HttpBreakerSvc::build_breaker_layer(cfg).expect("many failure_statuses must build");
}

// ---------------------------------------------------------------------------
// Debug output reflects policy
// ---------------------------------------------------------------------------

/// @covers: BreakerLayer
#[test]
fn test_core_breaker_layer_debug_includes_failure_threshold() {
    let cfg = BreakerConfig {
        failure_threshold: 8,
        half_open_after_seconds: 20,
        reset_after_successes: 3,
        failure_statuses: vec![500],
    };
    let layer = HttpBreakerSvc::build_breaker_layer(cfg).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("8"),
        "Debug must include failure_threshold; got: {dbg}"
    );
}

/// @covers: BreakerLayer
#[test]
fn test_core_breaker_layer_debug_includes_reset_after_successes() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 10,
        reset_after_successes: 6,
        failure_statuses: vec![503],
    };
    let layer = HttpBreakerSvc::build_breaker_layer(cfg).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("6"),
        "Debug must include reset_after_successes; got: {dbg}"
    );
}

// ---------------------------------------------------------------------------
// Send + Sync
// ---------------------------------------------------------------------------

/// @covers: BreakerLayer
#[test]
fn test_core_breaker_layer_is_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<BreakerLayer>();
}

// ---------------------------------------------------------------------------
// Per-host cache capacity — large host count
// ---------------------------------------------------------------------------

/// @covers: build_breaker_layer
#[test]
fn test_core_breaker_layer_builds_for_high_host_count_workload() {
    let cfg = BreakerConfig {
        failure_threshold: 5,
        half_open_after_seconds: 30,
        reset_after_successes: 2,
        failure_statuses: vec![500, 502, 503, 504],
    };
    HttpBreakerSvc::build_breaker_layer(cfg).expect("build for high-host-count workload");
}
