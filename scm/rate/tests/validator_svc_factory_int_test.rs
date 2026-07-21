//! Integration tests for the SAF `ValidatorFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{ConfigValidationRequest, RateConfig, ValidatorFactory};

/// @covers: ValidatorFactory
#[test]
fn test_validator_factory_create_returns_working_validator() {
    let validator = ValidatorFactory::create();
    // Valid config passes.
    assert!(validator
        .validate(ConfigValidationRequest {
            config: RateConfig::default()
        })
        .is_ok());
    // Sibling negative: a zero token rate must be rejected, proving the
    // factory-built validator actually inspects the config.
    assert!(validator
        .validate(ConfigValidationRequest {
            config: RateConfig {
                tokens_per_second: 0,
                burst_capacity: 10,
                per_host: false,
            }
        })
        .is_err());
}
