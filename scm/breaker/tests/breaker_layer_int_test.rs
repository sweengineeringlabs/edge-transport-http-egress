//! Integration tests for `api/types/breaker/layer.rs` — the public `BreakerLayer` type.
//!
//! Covers: constructability via `HttpBreakerSvc::build_breaker_layer(config)`, `Debug` output, and
//! `Send + Sync` bounds that allow the layer to be installed in a
//! `reqwest_middleware::ClientBuilder`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{BreakerConfig, BreakerLayer, HttpBreakerSvc};

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

/// @covers: build_breaker_layer
#[test]
fn test_breaker_layer_builds_from_custom_config() {
    let cfg = BreakerConfig {
        failure_threshold: 5,
        half_open_after_seconds: 30,
        reset_after_successes: 2,
        failure_statuses: vec![500, 503],
    };
    let _layer: BreakerLayer =
        HttpBreakerSvc::build_breaker_layer(cfg).expect("build() must succeed");
}

/// @covers: build_breaker_layer
#[test]
fn test_breaker_layer_builds_from_swe_default() {
    let _layer: BreakerLayer = HttpBreakerSvc::build_breaker_layer(BreakerConfig::default())
        .expect("build() must succeed");
}

// ---------------------------------------------------------------------------
// Debug output
// ---------------------------------------------------------------------------

/// @covers: BreakerLayer
#[test]
fn test_breaker_layer_debug_contains_type_name() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 60,
        reset_after_successes: 2,
        failure_statuses: vec![500],
    };
    let layer = HttpBreakerSvc::build_breaker_layer(cfg).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("BreakerLayer"),
        "Debug must name the type; got: {dbg}"
    );
}

/// @covers: BreakerLayer
#[test]
fn test_breaker_layer_debug_includes_failure_threshold() {
    let cfg = BreakerConfig {
        failure_threshold: 7,
        half_open_after_seconds: 10,
        reset_after_successes: 1,
        failure_statuses: vec![503],
    };
    let layer = HttpBreakerSvc::build_breaker_layer(cfg).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("7"),
        "Debug must include failure_threshold; got: {dbg}"
    );
}

/// @covers: BreakerLayer
#[test]
fn test_breaker_layer_debug_includes_half_open_wait() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 42,
        reset_after_successes: 2,
        failure_statuses: vec![],
    };
    let layer = HttpBreakerSvc::build_breaker_layer(cfg).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("42"),
        "Debug must include half_open_after_seconds; got: {dbg}"
    );
}

// ---------------------------------------------------------------------------
// Send + Sync — compile-time proof
// ---------------------------------------------------------------------------

/// @covers: BreakerLayer
#[test]
fn test_breaker_layer_is_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<BreakerLayer>();
}

// ---------------------------------------------------------------------------
// Two layers from different configs are independent
// ---------------------------------------------------------------------------

/// @covers: BreakerLayer
#[test]
fn test_two_breaker_layers_from_different_configs_are_independent() {
    let cfg_a = BreakerConfig {
        failure_threshold: 2,
        half_open_after_seconds: 5,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    };
    let cfg_b = BreakerConfig {
        failure_threshold: 20,
        half_open_after_seconds: 90,
        reset_after_successes: 5,
        failure_statuses: vec![503],
    };
    let layer_a = HttpBreakerSvc::build_breaker_layer(cfg_a).expect("build a");
    let layer_b = HttpBreakerSvc::build_breaker_layer(cfg_b).expect("build b");

    let dbg_a = format!("{layer_a:?}");
    let dbg_b = format!("{layer_b:?}");

    assert!(
        dbg_a.contains("2"),
        "layer_a must reflect threshold=2; got: {dbg_a}"
    );
    assert!(
        dbg_b.contains("20"),
        "layer_b must reflect threshold=20; got: {dbg_b}"
    );
}
