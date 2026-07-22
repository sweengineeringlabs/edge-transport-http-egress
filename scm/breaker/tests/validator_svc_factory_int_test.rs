//! Integration tests for [`ValidatorFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, ConfigValidationRequest, ValidatorFactory,
};

fn valid() -> BreakerConfig {
    BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 10,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    }
}

/// @covers: create
#[test]
fn test_create_produces_a_working_validator_happy() {
    let validator = ValidatorFactory::create();
    let result = validator.validate(ConfigValidationRequest { config: valid() });
    assert!(result.is_ok());
    // Sibling negative case in the same test: a single field flipped to
    // invalid on an otherwise-valid config must fail, proving is_ok() above
    // isn't just a stub that always succeeds regardless of input.
    let mut invalid = valid();
    invalid.failure_threshold = 0;
    let invalid_result = validator.validate(ConfigValidationRequest { config: invalid });
    assert!(invalid_result.is_err());
}

/// @covers: create
#[test]
fn test_create_produces_a_validator_that_rejects_bad_config_error() {
    let validator = ValidatorFactory::create();
    let mut config = valid();
    config.failure_threshold = 0;
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
