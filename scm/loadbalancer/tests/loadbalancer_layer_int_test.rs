#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `LoadbalancerLayer`.

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, LoadbalancerConfig, LoadbalancerLayer, LoadbalancerSvc, Strategy,
};

fn two_backend_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig {
                url: "https://api-1.internal".to_string(),
                weight: 1,
            },
            BackendConfig {
                url: "https://api-2.internal".to_string(),
                weight: 1,
            },
        ],
    }
}

/// @covers: LoadbalancerSvc::build_layer — produces a layer
#[test]
fn test_build_layer_returns_loadbalancer_layer() {
    let layer = LoadbalancerSvc::build_layer(two_backend_config()).expect("must build");
    let dbg = format!("{layer:?}");
    assert!(dbg.contains("LoadbalancerLayer"), "{dbg}");
}

/// @covers: LoadbalancerLayer is Send + Sync
#[test]
fn test_loadbalancer_layer_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<LoadbalancerLayer>();
}

/// @covers: LoadbalancerSvc::build_layer — empty backends rejected
#[test]
fn test_build_layer_fails_for_empty_backends() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    assert!(
        LoadbalancerSvc::build_layer(cfg).is_err(),
        "empty backends must fail"
    );
}

/// @covers: LoadbalancerSvc::build_layer — zero weight rejected
#[test]
fn test_build_layer_fails_for_zero_weight_backend() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api.test".to_string(),
            weight: 0,
        }],
    };
    assert!(
        LoadbalancerSvc::build_layer(cfg).is_err(),
        "zero weight must fail"
    );
}

/// @covers: build_loadbalancer_layer — SAF free function
#[test]
fn test_build_loadbalancer_layer_saf_function_builds_layer() {
    use edge_transport_http_egress_loadbalancer::build_loadbalancer_layer;
    let layer = build_loadbalancer_layer(two_backend_config()).expect("must build");
    let dbg = format!("{layer:?}");
    assert!(dbg.contains("LoadbalancerLayer"), "{dbg}");
}
