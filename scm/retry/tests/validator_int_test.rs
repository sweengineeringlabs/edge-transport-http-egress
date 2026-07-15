//! Integration tests for the `Validator` trait in `edge-transport-http-egress-retry`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::RetryConfigBuilder;

/// @covers: Validator — valid config passes validation
#[test]
fn test_validator_valid_config_passes() {
    let result = RetryConfigBuilder::new().build();
    assert!(result.is_ok(), "default config must pass validation");
}

/// @covers: Validator — zero multiplier is rejected
#[test]
fn test_validator_rejects_zero_multiplier() {
    let result = RetryConfigBuilder::new().multiplier(0.0).build();
    assert!(result.is_err(), "multiplier=0 must fail validation");
}

/// @covers: Validator — max_interval less than initial_interval is rejected
#[test]
fn test_validator_rejects_max_interval_below_initial() {
    let result = RetryConfigBuilder::new()
        .initial_interval_ms(1000)
        .max_interval_ms(100)
        .build();
    assert!(
        result.is_err(),
        "max_interval < initial_interval must fail validation"
    );
}
