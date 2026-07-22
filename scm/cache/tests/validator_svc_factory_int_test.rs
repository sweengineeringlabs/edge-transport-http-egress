//! Integration tests for [`ValidatorSvcFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, ConfigValidationRequest, ValidatorSvcFactory};

fn valid() -> CacheConfig {
    CacheConfig {
        default_ttl_seconds: 300,
        max_entries: 100,
        respect_cache_control: true,
        cache_private: false,
    }
}

/// @covers: create
#[test]
fn test_create_produces_a_working_validator_happy() {
    let validator = ValidatorSvcFactory::create();
    let result = validator.validate(ConfigValidationRequest { config: valid() });
    assert!(result.is_ok());
    // Sibling negative case in the same test: a single field flipped to
    // invalid on an otherwise-valid config must fail, proving is_ok() above
    // isn't just a stub that always succeeds regardless of input.
    let invalid = CacheConfig {
        max_entries: 0,
        ..valid()
    };
    let invalid_result = validator.validate(ConfigValidationRequest { config: invalid });
    assert!(invalid_result.is_err());
}

/// @covers: create
#[test]
fn test_create_produces_a_validator_that_rejects_bad_config_error() {
    let validator = ValidatorSvcFactory::create();
    let config = CacheConfig {
        max_entries: 0,
        ..valid()
    };
    let result = validator.validate(ConfigValidationRequest { config });
    assert!(result.is_err());
}

/// @covers: create
#[test]
fn test_create_produces_independent_instances_edge() {
    let first = ValidatorSvcFactory::create();
    let second = ValidatorSvcFactory::create();
    let r1 = first.validate(ConfigValidationRequest { config: valid() });
    let r2 = second.validate(ConfigValidationRequest { config: valid() });
    assert_eq!(r1.is_ok(), r2.is_ok());
}
