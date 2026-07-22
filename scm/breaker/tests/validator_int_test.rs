//! Integration tests for the `Validator` trait in `edge-transport-http-egress-breaker`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, BreakerError, ConfigValidationRequest, ValidatorFactory,
};

fn valid_config() -> BreakerConfig {
    BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 10,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    }
}

/// @covers: validate
#[test]
fn test_validate_well_formed_config_returns_ok_happy() {
    let validator = ValidatorFactory::create();
    let result = validator.validate(ConfigValidationRequest {
        config: valid_config(),
    });
    assert!(result.is_ok(), "a structurally valid config must pass");
    // Sibling negative case: a single field flipped to invalid on an
    // otherwise-valid config must fail, proving is_ok() above isn't just a
    // stub that always succeeds regardless of input.
    let mut invalid = valid_config();
    invalid.reset_after_successes = 0;
    let invalid_result = validator.validate(ConfigValidationRequest { config: invalid });
    assert!(
        invalid_result.is_err(),
        "an invalid config must be rejected, not silently accepted"
    );
}

/// @covers: validate
#[test]
fn test_validate_zero_failure_threshold_returns_err_error() {
    let validator = ValidatorFactory::create();
    let mut config = valid_config();
    config.failure_threshold = 0;
    let result = validator.validate(ConfigValidationRequest { config });
    assert!(
        matches!(result, Err(BreakerError::InvalidConfig(_))),
        "a zero failure_threshold must be rejected; got: {result:?}"
    );
}

/// @covers: validate
#[test]
fn test_validate_empty_failure_statuses_returns_err_edge() {
    let validator = ValidatorFactory::create();
    let mut config = valid_config();
    config.failure_statuses = vec![];
    let result = validator.validate(ConfigValidationRequest { config });
    assert!(
        matches!(result, Err(BreakerError::InvalidConfig(_))),
        "empty failure_statuses is an edge case that must still be rejected; got: {result:?}"
    );
}
