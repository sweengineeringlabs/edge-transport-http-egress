//! Dependency coverage test for `edge-transport-http-egress-loadbalancer`.
//! @covers: edge-transport-http-egress-loadbalancer
//!
//! Rule 95: `edge-transport-http-egress-loadbalancer` is used in `src/`
//! (feature = "loadbalancer", enabled by default) and must have integration
//! coverage with an explicit `use edge_transport_http_egress_loadbalancer::...`
//! import.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, BackendCountRequest, LoadbalancerConfig, LoadbalancerSvcProcessor, PoolMetrics,
    Strategy,
};

/// @covers: edge-transport-http-egress-loadbalancer
/// Verifies the `edge_transport_http_egress_loadbalancer` crate is accessible
/// and that its `LoadbalancerSvcProcessor::build_layer` entry point (the one
/// `transport_svc.rs` dispatches through when the `loadbalancer` feature is
/// enabled) genuinely applies the supplied config rather than a hardcoded
/// default: two configured backends must produce a pool of size two.
#[test]
fn transport_struct_dep_edge_transport_http_egress_loadbalancer_build_layer_int_test() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::Weighted,
        backends: vec![
            BackendConfig {
                url: "http://backend-a.example".to_string(),
                weight: 7,
            },
            BackendConfig {
                url: "http://backend-b.example".to_string(),
                weight: 3,
            },
        ],
    };
    let layer = LoadbalancerSvcProcessor::build_layer(cfg)
        .expect("valid loadbalancer config must build a layer");
    let count = layer
        .backend_count(BackendCountRequest)
        .expect("backend_count must succeed on a freshly built pool");
    assert_eq!(
        count.value, 2,
        "the pool built via the dependency's processor must carry both configured backends"
    );
}
