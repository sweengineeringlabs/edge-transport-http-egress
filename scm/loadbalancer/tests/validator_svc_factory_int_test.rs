//! Integration tests for [`ValidatorFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, ConfigValidationRequest, LoadbalancerConfig, Strategy, ValidatorFactory,
};

fn valid() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api.test".to_string(),
            weight: 1,
        }],
    }
}

/// @covers: create
#[test]
fn test_create_produces_a_working_validator_happy() {
    let validator = ValidatorFactory::create();
    let result = validator.validate(ConfigValidationRequest { config: valid() });
    assert!(result.is_ok());
    // Sibling negative case in the same test: an empty backend list must fail,
    // proving is_ok() above isn't a stub that always succeeds.
    let invalid = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    let invalid_result = validator.validate(ConfigValidationRequest { config: invalid });
    assert!(invalid_result.is_err());
}

/// @covers: create
#[test]
fn test_create_produces_a_validator_that_rejects_bad_config_error() {
    let validator = ValidatorFactory::create();
    let mut config = valid();
    config.backends[0].weight = 0;
    let result = validator.validate(ConfigValidationRequest { config });
    assert!(result.is_err());
}

/// @covers: create
#[test]
fn test_create_produces_independent_instances_edge() {
    let first = ValidatorFactory::create();
    let second = ValidatorFactory::create();
    let r1 = first.validate(ConfigValidationRequest { config: valid() });
    let r2 = second.validate(ConfigValidationRequest { config: valid() });
    assert_eq!(r1.is_ok(), r2.is_ok());
}
