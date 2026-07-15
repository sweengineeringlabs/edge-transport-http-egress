#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for the Validator contract (via SAF).

use edge_transport_http_egress_loadbalancer::{
    validate_loadbalancer_config, BackendConfig, LoadbalancerConfig, Strategy,
};

fn config_with(backends: Vec<BackendConfig>) -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends,
    }
}

fn backend(url: &str, weight: u32) -> BackendConfig {
    BackendConfig {
        url: url.to_string(),
        weight,
    }
}

/// @covers: validate_loadbalancer_config — valid config passes
#[test]
fn test_validate_loadbalancer_config_passes_for_valid_config() {
    let cfg = config_with(vec![backend("https://api.test", 1)]);
    assert!(
        validate_loadbalancer_config(&cfg).is_ok(),
        "valid config must pass"
    );
}

/// @covers: validate_loadbalancer_config — empty backend list
#[test]
fn test_validate_loadbalancer_config_fails_for_no_backends() {
    let cfg = config_with(vec![]);
    let err = validate_loadbalancer_config(&cfg).unwrap_err();
    assert!(err.contains("must not be empty"), "{err}");
}

/// @covers: validate_loadbalancer_config — zero weight rejected
#[test]
fn test_validate_loadbalancer_config_fails_for_zero_weight() {
    let cfg = config_with(vec![backend("https://api.test", 0)]);
    let err = validate_loadbalancer_config(&cfg).unwrap_err();
    assert!(err.contains("weight >= 1"), "{err}");
}

/// @covers: validate_loadbalancer_config — empty URL rejected
#[test]
fn test_validate_loadbalancer_config_fails_for_empty_url() {
    let cfg = config_with(vec![backend("", 1)]);
    let err = validate_loadbalancer_config(&cfg).unwrap_err();
    assert!(err.contains("non-empty url"), "{err}");
}

/// @covers: validate_loadbalancer_config — multiple valid backends passes
#[test]
fn test_validate_loadbalancer_config_passes_for_multiple_backends() {
    let cfg = config_with(vec![
        backend("https://api-1.internal", 2),
        backend("https://api-2.internal", 1),
    ]);
    assert!(
        validate_loadbalancer_config(&cfg).is_ok(),
        "multi-backend config must pass"
    );
}
