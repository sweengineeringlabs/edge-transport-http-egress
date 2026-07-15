#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for the `LoadbalancerSvc` factory.

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, LoadbalancerConfig, LoadbalancerSvc, Strategy,
};

fn one_backend() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api-1.internal".to_string(),
            weight: 1,
        }],
    }
}

/// @covers: LoadbalancerSvc::build_layer — happy path
#[test]
fn test_build_layer_succeeds_with_valid_config() {
    let result = LoadbalancerSvc::build_layer(one_backend());
    assert!(result.is_ok(), "valid config must build a layer");
}

/// @covers: LoadbalancerSvc::validate_config — happy path
#[test]
fn test_validate_config_passes_for_valid_config() {
    let cfg = one_backend();
    assert!(LoadbalancerSvc::validate_config(&cfg).is_ok());
}

/// @covers: LoadbalancerSvc::validate_config — sad path
#[test]
fn test_validate_config_fails_for_empty_backends() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    assert!(LoadbalancerSvc::validate_config(&cfg).is_err());
}

/// @covers: LoadbalancerSvc::build_layer — error contains useful message
#[test]
fn test_build_layer_error_describes_problem() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    let err = LoadbalancerSvc::build_layer(cfg).unwrap_err().to_string();
    assert!(!err.is_empty(), "error must have a non-empty message");
}

/// @covers: LoadbalancerSvc::create_config_builder
#[test]
fn test_create_config_builder_produces_builder() {
    let _builder = LoadbalancerSvc::create_config_builder();
}
