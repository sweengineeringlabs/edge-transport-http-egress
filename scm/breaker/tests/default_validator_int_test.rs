//! Integration tests for `DefaultValidator` — the crate's default `Validator`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, ConfigValidationRequest, ValidatorFactory,
};

/// @covers: create
#[test]
fn test_create_produces_a_default_validator() {
    let validator = ValidatorFactory::create();
    let result = validator.validate(ConfigValidationRequest {
        config: BreakerConfig::default(),
    });
    assert!(result.is_ok());
    // Sibling negative case in the same test: a single field flipped to
    // invalid on an otherwise-valid config must fail, proving is_ok() above
    // isn't just a stub that always succeeds regardless of input.
    let invalid = BreakerConfig {
        failure_threshold: 0,
        ..BreakerConfig::default()
    };
    let invalid_result = validator.validate(ConfigValidationRequest { config: invalid });
    assert!(invalid_result.is_err());
}
