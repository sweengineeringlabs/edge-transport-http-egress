//! Integration tests for `api/rate/rate_config_validator.rs` — the api/
//! structural counterpart of `core::rate::default_validator::DefaultValidator`.
//!
//! `DefaultValidator` itself has its own inline unit tests. From outside the
//! crate we verify the externally-observable effect: `ValidatorFactory`
//! rejects a structurally invalid `RateConfig`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{ConfigValidationRequest, RateConfig, ValidatorFactory};

/// @covers: validate
#[test]
fn test_validator_factory_rejects_zero_tokens_per_second_happy() {
    let validator = ValidatorFactory::create();
    let result = validator.validate(ConfigValidationRequest {
        config: RateConfig {
            tokens_per_second: 0,
            burst_capacity: 10,
            per_host: false,
        },
    });
    assert!(result.is_err());
}

/// @covers: validate
#[test]
fn test_validator_factory_accepts_minimum_valid_config_edge() {
    let validator = ValidatorFactory::create();
    let result = validator.validate(ConfigValidationRequest {
        config: RateConfig {
            tokens_per_second: 1,
            burst_capacity: 1,
            per_host: false,
        },
    });
    assert!(result.is_ok());
    // Sibling negative case in the same test: flipping tokens_per_second
    // back to 0 on this otherwise-minimal config must fail, proving the
    // is_ok() above isn't a stub that always succeeds.
    let invalid = validator.validate(ConfigValidationRequest {
        config: RateConfig {
            tokens_per_second: 0,
            burst_capacity: 1,
            per_host: false,
        },
    });
    assert!(invalid.is_err());
}
