//! Integration tests for `HttpBreakerSvc::build_breaker_layer`.

use edge_transport_http_egress_breaker::{BreakerConfig, HttpBreakerSvc};

/// @covers: build_breaker_layer
#[test]
fn test_build_breaker_layer_with_default_config_succeeds() {
    let result = HttpBreakerSvc::build_breaker_layer(BreakerConfig::default());
    assert!(
        result.is_ok(),
        "build_breaker_layer with default config must succeed"
    );
}
