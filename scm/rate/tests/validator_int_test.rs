//! Integration tests for the `Validator` trait in `edge-transport-http-egress-rate`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{HttpRateSvc, RateConfig};

/// @covers: Validator
#[test]
fn test_validator_rejects_zero_tokens_per_second() {
    // build_rate_layer invokes Validator::validate() internally.
    // A zero token rate must be rejected, proving the Validator is wired end-to-end.
    let cfg = RateConfig {
        tokens_per_second: 0,
        burst_capacity: 10,
        per_host: false,
    };
    let result = HttpRateSvc::build_rate_layer(cfg);
    assert!(
        result.is_err(),
        "zero tokens_per_second must fail validation"
    );
}

/// @covers: Validator
#[test]
fn test_validator_accepts_valid_config() {
    // Proves the Validator passes through a correct config without error.
    let cfg = RateConfig::default();
    let result = HttpRateSvc::build_rate_layer(cfg);
    assert!(result.is_ok(), "default config must pass validation");
}
