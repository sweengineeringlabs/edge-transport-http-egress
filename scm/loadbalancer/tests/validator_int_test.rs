#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for the `Validator` contract (via the SAF factory).

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, ConfigValidationRequest, LoadbalancerConfig, LoadbalancerMiddlewareError,
    Strategy, ValidatorFactory,
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

/// @covers: validate
#[test]
fn test_validate_loadbalancer_config_passes_for_valid_config() {
    let validator = ValidatorFactory::create();
    let cfg = config_with(vec![backend("https://api.test", 1)]);
    assert!(validator
        .validate(ConfigValidationRequest { config: cfg })
        .is_ok());
    // Sibling negative case: an empty backend list must be rejected, proving
    // the is_ok() above isn't a stub that accepts everything.
    let empty = config_with(vec![]);
    assert!(validator
        .validate(ConfigValidationRequest { config: empty })
        .is_err());
}

/// @covers: validate
#[test]
fn test_validate_loadbalancer_config_fails_for_no_backends() {
    let validator = ValidatorFactory::create();
    let err = validator
        .validate(ConfigValidationRequest {
            config: config_with(vec![]),
        })
        .unwrap_err();
    assert!(
        matches!(err, LoadbalancerMiddlewareError::InvalidConfig(ref m) if m.contains("must not be empty")),
        "{err}"
    );
}

/// @covers: validate
#[test]
fn test_validate_loadbalancer_config_fails_for_zero_weight() {
    let validator = ValidatorFactory::create();
    let err = validator
        .validate(ConfigValidationRequest {
            config: config_with(vec![backend("https://api.test", 0)]),
        })
        .unwrap_err();
    assert!(
        matches!(err, LoadbalancerMiddlewareError::InvalidConfig(ref m) if m.contains("weight >= 1")),
        "{err}"
    );
}

/// @covers: validate
#[test]
fn test_validate_loadbalancer_config_fails_for_empty_url() {
    let validator = ValidatorFactory::create();
    let err = validator
        .validate(ConfigValidationRequest {
            config: config_with(vec![backend("", 1)]),
        })
        .unwrap_err();
    assert!(
        matches!(err, LoadbalancerMiddlewareError::InvalidConfig(ref m) if m.contains("non-empty url")),
        "{err}"
    );
}

/// @covers: validate
#[test]
fn test_validate_loadbalancer_config_passes_for_multiple_backends() {
    let validator = ValidatorFactory::create();
    let cfg = config_with(vec![
        backend("https://api-1.internal", 2),
        backend("https://api-2.internal", 1),
    ]);
    assert!(validator
        .validate(ConfigValidationRequest {
            config: cfg.clone()
        })
        .is_ok());
    // Sibling negative: flip one backend's weight to 0 — must now fail.
    let mut invalid = cfg;
    invalid.backends[1].weight = 0;
    assert!(validator
        .validate(ConfigValidationRequest { config: invalid })
        .is_err());
}
