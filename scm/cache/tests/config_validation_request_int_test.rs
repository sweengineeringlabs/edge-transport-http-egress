//! Integration tests for `ConfigValidationRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, ConfigValidationRequest, ValidatorSvcFactory};

fn valid_config() -> CacheConfig {
    CacheConfig {
        default_ttl_seconds: 300,
        max_entries: 100,
        respect_cache_control: true,
        cache_private: false,
    }
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_config_field_drives_ok_happy() {
    let validator = ValidatorSvcFactory::create();
    let result = validator.validate(ConfigValidationRequest {
        config: valid_config(),
    });
    assert!(result.is_ok());
    // Sibling negative case in the same test: a single field flipped to
    // invalid on an otherwise-valid config must fail, proving is_ok() above
    // isn't just a stub that always succeeds regardless of input.
    let invalid = CacheConfig {
        max_entries: 0,
        ..valid_config()
    };
    let invalid_result = validator.validate(ConfigValidationRequest { config: invalid });
    assert!(invalid_result.is_err());
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_config_field_drives_err_error() {
    let validator = ValidatorSvcFactory::create();
    let config = CacheConfig {
        max_entries: 0,
        ..valid_config()
    };
    let result = validator.validate(ConfigValidationRequest { config });
    assert!(
        result.is_err(),
        "ConfigValidationRequest's config field must be the thing actually validated"
    );
}
