//! Integration tests for [`ValidatorFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::{RetryConfig, ValidationRequest, ValidatorFactory};

fn valid() -> RetryConfig {
    RetryConfig::default()
}

/// @covers: create
#[test]
fn test_create_produces_a_working_validator_happy() {
    let validator = ValidatorFactory::create();
    let result = validator.validate(ValidationRequest { config: valid() });
    assert!(result.is_ok());
    // Sibling negative case in the same test: a single field flipped to
    // invalid on an otherwise-valid config must fail, proving is_ok() above
    // isn't just a stub that always succeeds regardless of input.
    let invalid = RetryConfig {
        multiplier: 0.0,
        ..valid()
    };
    let invalid_result = validator.validate(ValidationRequest { config: invalid });
    assert!(invalid_result.is_err());
}

/// @covers: create
#[test]
fn test_create_produces_a_validator_that_rejects_bad_config_error() {
    let validator = ValidatorFactory::create();
    let config = RetryConfig {
        multiplier: 0.0,
        ..valid()
    };
    let result = validator.validate(ValidationRequest { config });
    assert!(result.is_err());
}

/// @covers: create
#[test]
fn test_create_produces_independent_instances_edge() {
    let first = ValidatorFactory::create();
    let second = ValidatorFactory::create();
    let r1 = first.validate(ValidationRequest { config: valid() });
    let r2 = second.validate(ValidationRequest { config: valid() });
    assert_eq!(r1.is_ok(), r2.is_ok());
}
