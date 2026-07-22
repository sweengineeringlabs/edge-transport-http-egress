//! Integration tests for `BackendCountResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, BackendCountRequest, LoadbalancerConfig, LoadbalancerSvcProcessor, PoolMetrics,
    Strategy,
};

/// @covers: BackendCountResponse
#[test]
fn test_backend_count_response_value_matches_backend_list_len_happy() {
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
        ],
    };
    let layer = LoadbalancerSvcProcessor::build_layer(cfg).expect("build must succeed");
    let resp = layer
        .backend_count(BackendCountRequest)
        .expect("infallible");
    assert_eq!(resp.value, 3);
}

/// @covers: BackendCountResponse
#[test]
fn test_backend_count_response_value_is_a_plain_usize_edge() {
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
    let value: usize = resp.value;
    assert_eq!(value, 1);
}
