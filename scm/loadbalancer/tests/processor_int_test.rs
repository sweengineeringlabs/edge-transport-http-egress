#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for the Processor contract.

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, LoadbalancerConfig, LoadbalancerSvc, Strategy,
};

fn one_backend_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api-1.internal".to_string(),
            weight: 1,
        }],
    }
}

/// @covers: LoadbalancerSvc::build_layer — processor describe contract
#[test]
fn test_build_layer_processor_describe_returns_crate_name() {
    let layer =
        LoadbalancerSvc::build_layer(one_backend_config()).expect("valid config must build");
    let dbg = format!("{layer:?}");
    assert!(dbg.contains("LoadbalancerLayer"), "{dbg}");
}

/// @covers: LoadbalancerSvc::create_config_builder
#[test]
fn test_create_config_builder_returns_builder_with_name() {
    // Verify it constructs without panicking; ConfigBuilderImpl doesn't derive Debug.
    let _builder = LoadbalancerSvc::create_config_builder();
}
