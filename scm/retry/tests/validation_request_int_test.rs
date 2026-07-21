//! Integration tests for `ValidationRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::{RetryConfig, ValidationRequest, ValidatorFactory};

/// @covers: ValidationRequest
#[test]
fn test_validation_request_config_field_drives_ok_happy() {
    let validator = ValidatorFactory::create();
    let result = validator.validate(ValidationRequest {
        config: RetryConfig::default(),
    });
    assert!(result.is_ok());
    // Sibling negative case in the same test: a single field flipped to
    // invalid on an otherwise-valid config must fail, proving is_ok() above
    // isn't just a stub that always succeeds regardless of input.
    let invalid = ValidationRequest {
        config: RetryConfig {
            multiplier: 0.0,
            ..RetryConfig::default()
        },
    };
    assert!(validator.validate(invalid).is_err());
}

/// @covers: ValidationRequest
#[test]
fn test_validation_request_config_field_drives_err_error() {
    let validator = ValidatorFactory::create();
    let invalid = ValidationRequest {
        config: RetryConfig {
            multiplier: 0.0,
            ..RetryConfig::default()
        },
    };
    let result = validator.validate(invalid);
    assert!(
        result.is_err(),
        "ValidationRequest's config field must be the thing actually validated"
    );
}
