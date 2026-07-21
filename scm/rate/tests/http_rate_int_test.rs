//! Integration tests for the public `RateLayerRateMetrics` middleware surface.
//!
//! `RateLayerRateMetrics` is installed in a `reqwest_middleware::ClientBuilder`, so it
//! must be usable and satisfy `Send + Sync` when shared behind an `Arc`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_rate::{HttpRateSvcProcessor, RateConfig, RateLayerRateMetrics};

// ---------------------------------------------------------------------------
// Constructed layer is usable
// ---------------------------------------------------------------------------

/// A `RateLayerRateMetrics` produced by the builder must be ready to use.
#[test]
fn test_rate_layer_built_from_builder_is_usable() {
    let cfg = RateConfig {
        tokens_per_second: 10,
        burst_capacity: 20,
        per_host: true,
    };
    let layer: RateLayerRateMetrics =
        HttpRateSvcProcessor::build_rate_layer(cfg).expect("build() must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        !dbg.is_empty(),
        "RateLayerRateMetrics Debug must produce non-empty output"
    );
}

// ---------------------------------------------------------------------------
// Send + Sync — runtime proof via a real thread boundary
// ---------------------------------------------------------------------------

/// `Arc<RateLayerRateMetrics>` must be `Send + Sync` so the same layer can be shared
/// across the worker threads of a `reqwest` client. Prove it at runtime by
/// moving the `Arc` onto a spawned OS thread and asserting the Debug output
/// observed there reflects the configured (non-default) policy.
#[test]
fn test_arc_rate_layer_is_send_and_sync() {
    let layer = Arc::new(
        HttpRateSvcProcessor::build_rate_layer(RateConfig {
            tokens_per_second: 88,
            burst_capacity: 176,
            per_host: false,
        })
        .expect("build"),
    );
    let moved = Arc::clone(&layer);
    let dbg = std::thread::spawn(move || format!("{moved:?}"))
        .join()
        .expect("worker thread must join");
    assert!(
        dbg.contains("88") && dbg.contains("176"),
        "Debug read on the worker thread must reflect the config; got: {dbg}"
    );
    // The original Arc is still usable on this thread — sharing, not moving.
    assert_eq!(
        Arc::strong_count(&layer),
        1,
        "worker thread dropped its clone"
    );
}
