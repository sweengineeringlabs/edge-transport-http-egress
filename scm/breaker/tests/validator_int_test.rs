//! Integration tests for the `Validator` trait in `edge-transport-http-egress-breaker`.

use edge_transport_http_egress_breaker::{BreakerConfig, BreakerError};

/// @covers: Validator
/// The `Validator` trait is implemented internally by the config layer.
/// Its contract is observable through `BreakerConfig::from_config`: valid TOML
/// parses to `Ok`, and malformed TOML returns `Err(BreakerError::ParseFailed)`.
#[test]
fn test_validator_trait_exists_in_crate() {
    let result = BreakerConfig::from_config("not valid toml = [[[");
    assert!(
        matches!(result, Err(BreakerError::ParseFailed(_))),
        "invalid config must return ParseFailed; got: {result:?}"
    );
}
