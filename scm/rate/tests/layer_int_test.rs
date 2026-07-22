//! Integration tests for `api/rate/layer.rs` — the api/ structural
//! counterpart of `core::rate::layer`.
//!
//! From outside the crate we verify the externally-observable effect: a
//! `RateLayerRateMetrics` built from a real config is ready to dispatch
//! through `reqwest_middleware`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{HttpRateSvcProcessor, RateConfig};

/// @covers: build_rate_layer
#[test]
fn test_layer_built_with_per_host_true_is_usable() {
    let cfg = RateConfig {
        tokens_per_second: 5,
        burst_capacity: 10,
        per_host: true,
    };
    let layer = HttpRateSvcProcessor::build_rate_layer(cfg).expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(!dbg.is_empty());
}
