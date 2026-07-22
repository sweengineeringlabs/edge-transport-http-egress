#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `LoadbalancerLayerPoolMetrics`.

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, LoadbalancerConfig, LoadbalancerSvcProcessor, Strategy,
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

/// @covers: build_layer
#[test]
fn test_build_layer_returns_loadbalancer_layer() {
    let layer = LoadbalancerSvcProcessor::build_layer(two_backend_config()).expect("must build");
    let dbg = format!("{layer:?}");
    assert!(dbg.contains("LoadbalancerLayerPoolMetrics"), "{dbg}");
}

/// `LoadbalancerLayerPoolMetrics` must survive being moved across a real thread boundary
/// (the `reqwest_middleware::Middleware` bounds require `Send + Sync`); this
/// spawns it onto a multi-thread runtime and asserts on its real `Debug`
/// output produced on the other thread.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_loadbalancer_layer_usable_across_threads() {
    let layer = LoadbalancerSvcProcessor::build_layer(two_backend_config()).expect("must build");
    let dbg = tokio::spawn(async move { format!("{layer:?}") })
        .await
        .expect("spawned task joins");
    assert!(dbg.contains("LoadbalancerLayerPoolMetrics"), "{dbg}");
}

/// @covers: build_layer
#[test]
fn test_build_layer_fails_for_empty_backends() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    assert!(
        LoadbalancerSvcProcessor::build_layer(cfg).is_err(),
        "empty backends must fail"
    );
}

/// @covers: build_layer
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
        LoadbalancerSvcProcessor::build_layer(cfg).is_err(),
        "zero weight must fail"
    );
}
