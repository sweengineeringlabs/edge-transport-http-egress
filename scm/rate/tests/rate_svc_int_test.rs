//! Integration tests for `HttpRateSvc::build_rate_layer`.

use edge_transport_http_egress_rate::{HttpRateSvc, RateConfig};

/// @covers: build_rate_layer
#[test]
fn test_build_rate_layer_with_default_config_succeeds() {
    let result = HttpRateSvc::build_rate_layer(RateConfig::default());
    assert!(
        result.is_ok(),
        "build_rate_layer with default config must succeed"
    );
}
