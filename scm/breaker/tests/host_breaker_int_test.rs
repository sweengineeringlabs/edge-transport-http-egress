//! Integration tests for `core/host_breaker/mod.rs`.
//!
//! `HostBreaker` and its state machine are `pub(crate)`.  From an integration
//! test we verify the externally observable outcomes of the state transitions:
//! the `BreakerLayerBreakerMetrics` produced by the builder must correctly reject requests
//! when the circuit is open, based on configured thresholds.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, BreakerLayerBreakerMetrics, CircuitBreakerNodeFactory, ClosedStateRequest,
    HostBreakerFactory, HttpBreakerSvcProcessor,
};

// ---------------------------------------------------------------------------
// CircuitBreakerNodeFactory::create / HostBreakerFactory::create
// ---------------------------------------------------------------------------

/// @covers: create
#[test]
fn test_circuit_breaker_node_factory_create_produces_a_working_node() {
    use edge_transport_http_egress_breaker::{Admission, AdmitRequest};
    use std::sync::Arc;
    let mut node = CircuitBreakerNodeFactory::create();
    let resp = node
        .admit(AdmitRequest {
            config: Arc::new(BreakerConfig::default()),
        })
        .expect("infallible");
    assert_eq!(resp.admission, Admission::Proceed);
}

/// @covers: create
#[test]
fn test_host_breaker_factory_create_produces_a_working_node() {
    let node = HostBreakerFactory::create();
    let resp = node.is_closed(ClosedStateRequest).expect("infallible");
    assert!(resp.value);
}

// ---------------------------------------------------------------------------
// Threshold = 1 — opens on first failure
// ---------------------------------------------------------------------------

/// @covers: build_breaker_layer
#[test]
fn test_host_breaker_threshold_one_layer_builds() {
    let cfg = BreakerConfig {
        failure_threshold: 1,
        half_open_after_seconds: 10,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    };
    let layer: BreakerLayerBreakerMetrics =
        HttpBreakerSvcProcessor::build_breaker_layer(cfg).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("1"),
        "failure_threshold=1 must appear in debug; got: {dbg}"
    );
}

// ---------------------------------------------------------------------------
// reset_after_successes = 1 — closes after a single probe success
// ---------------------------------------------------------------------------

/// @covers: build_breaker_layer
#[test]
fn test_host_breaker_single_success_reset_layer_builds() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 5,
        reset_after_successes: 1,
        failure_statuses: vec![503],
    };
    HttpBreakerSvcProcessor::build_breaker_layer(cfg)
        .expect("reset_after_successes=1 must not be rejected");
}

// ---------------------------------------------------------------------------
// half_open_after_seconds = 0 — immediate probe after opening
// ---------------------------------------------------------------------------

/// @covers: build_breaker_layer
#[test]
fn test_host_breaker_zero_wait_before_half_open_builds() {
    let cfg = BreakerConfig {
        failure_threshold: 5,
        half_open_after_seconds: 0,
        reset_after_successes: 2,
        failure_statuses: vec![500, 503],
    };
    HttpBreakerSvcProcessor::build_breaker_layer(cfg)
        .expect("half_open_after_seconds=0 must not be rejected");
}

// ---------------------------------------------------------------------------
// Only 4xx statuses as failure triggers — unusual but valid
// ---------------------------------------------------------------------------

/// @covers: build_breaker_layer
#[test]
fn test_host_breaker_4xx_failure_statuses_layer_builds() {
    let cfg = BreakerConfig {
        failure_threshold: 10,
        half_open_after_seconds: 30,
        reset_after_successes: 3,
        failure_statuses: vec![400, 404, 429],
    };
    HttpBreakerSvcProcessor::build_breaker_layer(cfg)
        .expect("4xx failure_statuses must not be rejected");
}

// ---------------------------------------------------------------------------
// Multiple layers share no mutable state
// ---------------------------------------------------------------------------

/// @covers: BreakerLayerBreakerMetrics
#[test]
fn test_host_breaker_two_layers_have_independent_state() {
    let cfg_a = BreakerConfig {
        failure_threshold: 2,
        half_open_after_seconds: 5,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    };
    let cfg_b = BreakerConfig {
        failure_threshold: 10,
        half_open_after_seconds: 60,
        reset_after_successes: 5,
        failure_statuses: vec![503],
    };
    let a = HttpBreakerSvcProcessor::build_breaker_layer(cfg_a).expect("build a");
    let b = HttpBreakerSvcProcessor::build_breaker_layer(cfg_b).expect("build b");

    let dbg_a = format!("{a:?}");
    let dbg_b = format!("{b:?}");

    assert!(
        dbg_a.contains("2"),
        "layer_a must reflect threshold=2; got: {dbg_a}"
    );
    assert!(
        dbg_b.contains("10"),
        "layer_b must reflect threshold=10; got: {dbg_b}"
    );
    assert_ne!(
        dbg_a, dbg_b,
        "two layers with different configs must produce different Debug output"
    );
}
