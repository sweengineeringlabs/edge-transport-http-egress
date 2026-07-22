//! Integration tests for [`PoolMetricsFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, BackendCountRequest, LoadbalancerConfig, LoadbalancerSvcProcessor,
    PoolMetricsFactory, Strategy,
};

/// @covers: from_layer
#[test]
fn test_from_layer_upcasts_to_backend_count_happy() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig {
                url: "https://api-1.test".to_string(),
                weight: 1,
            },
            BackendConfig {
                url: "https://api-2.test".to_string(),
                weight: 1,
            },
        ],
    };
    let layer = LoadbalancerSvcProcessor::build_layer(cfg).expect("build ok");
    let pool = PoolMetricsFactory::from_layer(layer);
    let resp = pool
        .backend_count(BackendCountRequest)
        .expect("upcast must genuinely work");
    assert_eq!(resp.value, 2);
}

/// @covers: from_layer
#[test]
fn test_from_layer_reflects_a_single_backend_edge() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api.test".to_string(),
            weight: 1,
        }],
    };
    let layer = LoadbalancerSvcProcessor::build_layer(cfg).expect("build ok");
    let pool = PoolMetricsFactory::from_layer(layer);
    let resp = pool
        .backend_count(BackendCountRequest)
        .expect("must succeed");
    assert_eq!(resp.value, 1);
}

/// @covers: from_layer
#[test]
fn test_from_layer_reflects_many_backends_edge() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig {
                url: "https://api-1.test".to_string(),
                weight: 1,
            },
            BackendConfig {
                url: "https://api-2.test".to_string(),
                weight: 1,
            },
            BackendConfig {
                url: "https://api-3.test".to_string(),
                weight: 1,
            },
            BackendConfig {
                url: "https://api-4.test".to_string(),
                weight: 1,
            },
        ],
    };
    let layer = LoadbalancerSvcProcessor::build_layer(cfg).expect("build ok");
    let pool = PoolMetricsFactory::from_layer(layer);
    let resp = pool
        .backend_count(BackendCountRequest)
        .expect("must succeed");
    assert_eq!(resp.value, 4);
}
