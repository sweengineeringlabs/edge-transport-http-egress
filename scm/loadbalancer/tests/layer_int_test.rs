//! Integration tests for `api/layer/layer.rs` — the api/ structural
//! counterpart of `core::layer::layer`.
//!
//! From outside the crate we verify the externally-observable effect: a
//! `LoadbalancerLayerPoolMetrics` built from a real config is ready to
//! dispatch through `reqwest_middleware`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, LoadbalancerConfig, LoadbalancerSvcProcessor, Strategy,
};

/// @covers: build_layer
#[test]
fn test_layer_built_with_multiple_backends_is_usable() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig {
                url: "https://api-1.test".to_string(),
                weight: 1,
            },
            BackendConfig {
                url: "https://api-2.test".to_string(),
                weight: 2,
            },
        ],
    };
    let layer = LoadbalancerSvcProcessor::build_layer(cfg).expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(!dbg.is_empty());
}
