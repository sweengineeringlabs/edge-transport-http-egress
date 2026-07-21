//! Integration tests for `api/rate_layer.rs` — the public `RateLayerRateMetrics` type.
//!
//! Covers: constructability via `HttpRateSvcProcessor::build_rate_layer(config)`, `Debug` output, and
//! `Send + Sync` bounds required by `reqwest_middleware::ClientBuilder::with()`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{HttpRateSvcProcessor, RateConfig, RateLayerRateMetrics};

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

/// The nominal construction path must succeed.
#[test]
fn test_rate_layer_builds_from_custom_config() {
    let cfg = RateConfig {
        tokens_per_second: 20,
        burst_capacity: 40,
        per_host: true,
    };
    let _layer: RateLayerRateMetrics =
        HttpRateSvcProcessor::build_rate_layer(cfg).expect("build() must succeed");
}

/// Building from the SWE default must also succeed.
#[test]
fn test_rate_layer_builds_from_swe_default() {
    let _layer: RateLayerRateMetrics =
        HttpRateSvcProcessor::build_rate_layer(RateConfig::default())
            .expect("build() must succeed");
}

// ---------------------------------------------------------------------------
// Debug output
// ---------------------------------------------------------------------------

/// `RateLayerRateMetrics::fmt` must include the type name.
#[test]
fn test_rate_layer_debug_contains_type_name() {
    let cfg = RateConfig {
        tokens_per_second: 10,
        burst_capacity: 20,
        per_host: false,
    };
    let layer = HttpRateSvcProcessor::build_rate_layer(cfg).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("RateLayerRateMetrics"),
        "Debug must name the type; got: {dbg}"
    );
}

/// `tokens_per_second` must appear in `Debug` output.
#[test]
fn test_rate_layer_debug_includes_tokens_per_second() {
    let cfg = RateConfig {
        tokens_per_second: 77,
        burst_capacity: 100,
        per_host: false,
    };
    let layer = HttpRateSvcProcessor::build_rate_layer(cfg).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("77"),
        "Debug must include tokens_per_second; got: {dbg}"
    );
}

/// `burst_capacity` must appear in `Debug` output.
#[test]
fn test_rate_layer_debug_includes_burst_capacity() {
    let cfg = RateConfig {
        tokens_per_second: 5,
        burst_capacity: 333,
        per_host: false,
    };
    let layer = HttpRateSvcProcessor::build_rate_layer(cfg).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("333"),
        "Debug must include burst_capacity; got: {dbg}"
    );
}

// ---------------------------------------------------------------------------
// Send + Sync — runtime proof via a real thread boundary
// ---------------------------------------------------------------------------

/// `RateLayerRateMetrics` must satisfy `Send + Sync` — required so it can be shared behind
/// an `Arc` across the threads of a `reqwest` client. Prove it at runtime by
/// moving an `Arc<RateLayerRateMetrics>` onto a spawned OS thread (which requires
/// `RateLayerRateMetrics: Send + Sync`) and reading its Debug output there.
#[test]
fn test_rate_layer_is_send_and_sync() {
    use std::sync::Arc;
    let layer = Arc::new(
        HttpRateSvcProcessor::build_rate_layer(RateConfig {
            tokens_per_second: 61,
            burst_capacity: 122,
            per_host: false,
        })
        .expect("build"),
    );
    let moved = Arc::clone(&layer);
    let dbg = std::thread::spawn(move || format!("{moved:?}"))
        .join()
        .expect("worker thread must join");
    assert!(
        dbg.contains("61") && dbg.contains("122"),
        "Debug read on the worker thread must reflect the config; got: {dbg}"
    );
}

// ---------------------------------------------------------------------------
// Two layers from different configs are independent
// ---------------------------------------------------------------------------

/// Two independently built layers must not share state.
#[test]
fn test_two_rate_layers_from_different_configs_are_independent() {
    let cfg_a = RateConfig {
        tokens_per_second: 5,
        burst_capacity: 10,
        per_host: false,
    };
    let cfg_b = RateConfig {
        tokens_per_second: 500,
        burst_capacity: 1000,
        per_host: true,
    };
    let layer_a = HttpRateSvcProcessor::build_rate_layer(cfg_a).expect("build a");
    let layer_b = HttpRateSvcProcessor::build_rate_layer(cfg_b).expect("build b");

    let dbg_a = format!("{layer_a:?}");
    let dbg_b = format!("{layer_b:?}");

    assert!(
        dbg_a.contains("5"),
        "layer_a must reflect rate=5; got: {dbg_a}"
    );
    assert!(
        dbg_b.contains("500"),
        "layer_b must reflect rate=500; got: {dbg_b}"
    );
    assert_ne!(
        dbg_a, dbg_b,
        "two layers with different configs must produce different Debug output"
    );
}
