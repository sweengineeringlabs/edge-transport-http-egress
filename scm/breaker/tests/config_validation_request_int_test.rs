//! Integration tests for `ConfigValidationRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, ConfigValidationRequest, ValidatorFactory,
};

fn valid_config() -> BreakerConfig {
    BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 10,
        reset_after_successes: 1,
        failure_statuses: vec![500],
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
    // Sibling negative case in the same test: a single field flipped to
    // invalid on an otherwise-valid config must fail, proving is_ok() above
    // isn't just a stub that always succeeds regardless of input.
    let mut invalid = valid_config();
    invalid.failure_threshold = 0;
    let invalid_result = validator.validate(ConfigValidationRequest { config: invalid });
    assert!(invalid_result.is_err());
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_config_field_drives_err_error() {
    let validator = ValidatorFactory::create();
    let mut config = valid_config();
    config.failure_threshold = 0;
    let result = validator.validate(ConfigValidationRequest { config });
    assert!(
        result.is_err(),
        "ConfigValidationRequest's config field must be the thing actually validated"
    );
}
