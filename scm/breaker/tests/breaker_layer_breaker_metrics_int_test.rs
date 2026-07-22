//! Integration tests for `BreakerLayerBreakerMetrics` — the crate's
//! circuit-breaker middleware type.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, BreakerLayerBreakerMetrics, HttpBreakerSvcProcessor,
};

/// @covers: BreakerLayerBreakerMetrics
#[test]
fn test_breaker_layer_debug_reflects_configured_threshold_happy() {
    let cfg = BreakerConfig {
        failure_threshold: 11,
        half_open_after_seconds: 5,
        reset_after_successes: 2,
        failure_statuses: vec![500],
    };
    let layer: BreakerLayerBreakerMetrics =
        HttpBreakerSvcProcessor::build_breaker_layer(cfg).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("11"),
        "Debug output must reflect the configured failure_threshold: {dbg}"
    );
}

/// @covers: BreakerLayerBreakerMetrics
#[test]
fn test_breaker_layer_is_coercible_to_boxed_send_sync_edge() {
    let cfg = BreakerConfig::default();
    let layer: BreakerLayerBreakerMetrics =
        HttpBreakerSvcProcessor::build_breaker_layer(cfg).expect("build");
    let _boxed: Box<dyn Send + Sync> = Box::new(layer);
}
