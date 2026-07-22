#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for the `LoadbalancerSvcProcessor` factory.

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, LoadbalancerConfig, LoadbalancerMiddlewareError, LoadbalancerSvcProcessor,
    Strategy,
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

/// @covers: build_layer
#[test]
fn test_build_layer_succeeds_with_valid_config() {
    let layer = LoadbalancerSvcProcessor::build_layer(one_backend()).expect("valid config builds");
    assert!(format!("{layer:?}").contains("LoadbalancerLayerPoolMetrics"));
    // Sibling negative: empty backends must be rejected.
    let empty = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    assert!(LoadbalancerSvcProcessor::build_layer(empty).is_err());
}

/// @covers: validate_config
#[test]
fn test_validate_config_passes_for_valid_config() {
    assert!(LoadbalancerSvcProcessor::validate_config(&one_backend()).is_ok());
    // Sibling negative: empty backends must fail.
    let empty = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    assert!(LoadbalancerSvcProcessor::validate_config(&empty).is_err());
}

/// @covers: validate_config
#[test]
fn test_validate_config_fails_for_empty_backends() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    assert!(matches!(
        LoadbalancerSvcProcessor::validate_config(&cfg),
        Err(LoadbalancerMiddlewareError::InvalidConfig(_))
    ));
}

/// @covers: build_layer
#[test]
fn test_build_layer_error_describes_problem() {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    let err = LoadbalancerSvcProcessor::build_layer(cfg)
        .unwrap_err()
        .to_string();
    assert!(err.contains("invalid configuration"), "{err}");
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_produces_seeded_builder() {
    let builder = LoadbalancerSvcProcessor::create_config_builder();
    assert_eq!(builder.name(), "edge-transport-http-egress-loadbalancer");
}
