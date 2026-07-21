//! Integration tests for `ConfigValidationRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, ConfigValidationRequest, LoadbalancerConfig, Strategy, ValidatorFactory,
};

fn valid_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "https://api.test".to_string(),
            weight: 1,
        }],
    }
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_config_field_drives_ok_happy() {
    let validator = ValidatorFactory::create();
    let result = validator.validate(ConfigValidationRequest {
        config: valid_config(),
    });
    assert!(result.is_ok());
    // Sibling negative case in the same test: an empty backend list on an
    // otherwise-valid config must fail, proving is_ok() above isn't just a
    // stub that always succeeds regardless of input.
    let invalid = LoadbalancerConfig {
        backends: vec![],
        ..valid_config()
    };
    let invalid_result = validator.validate(ConfigValidationRequest { config: invalid });
    assert!(invalid_result.is_err());
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_config_field_drives_err_error() {
    let validator = ValidatorFactory::create();
    let config = LoadbalancerConfig {
        backends: vec![],
        ..valid_config()
    };
    let result = validator.validate(ConfigValidationRequest { config });
    assert!(
        result.is_err(),
        "ConfigValidationRequest's config field must be the thing actually validated"
    );
}
