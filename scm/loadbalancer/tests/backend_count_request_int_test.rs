//! Integration tests for `BackendCountRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, BackendCountRequest, LoadbalancerConfig, LoadbalancerSvcProcessor, PoolMetrics,
    Strategy,
};

/// @covers: backend_count
#[test]
fn test_backend_count_request_produces_the_configured_count_happy() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api.test".to_string(),
            weight: 1,
        }],
    };
    let layer = LoadbalancerSvcProcessor::build_layer(cfg).expect("build must succeed");
    let resp = layer
        .backend_count(BackendCountRequest)
        .expect("infallible");
    assert_eq!(resp.value, 1);
}

/// @covers: backend_count
#[test]
fn test_backend_count_request_is_reusable_across_calls_edge() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api.test".to_string(),
            weight: 1,
        }],
    };
    let layer = LoadbalancerSvcProcessor::build_layer(cfg).expect("build must succeed");
    let a = layer
        .backend_count(BackendCountRequest)
        .expect("infallible");
    let b = layer
        .backend_count(BackendCountRequest)
        .expect("infallible");
    assert_eq!(a.value, b.value);
}
