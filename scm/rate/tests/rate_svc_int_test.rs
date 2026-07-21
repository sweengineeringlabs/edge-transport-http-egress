//! Integration tests for `HttpRateSvcProcessor::build_rate_layer`.

use edge_transport_http_egress_rate::{HttpRateSvcProcessor, RateConfig};

/// @covers: build_rate_layer
#[test]
fn test_build_rate_layer_with_default_config_succeeds() {
    let result = HttpRateSvcProcessor::build_rate_layer(RateConfig::default());
    assert!(
        result.is_ok(),
        "build_rate_layer with default config must succeed"
    );
    // Sibling negative: an invalid config (zero token rate) must be rejected,
    // proving build_rate_layer inspects the config rather than always
    // returning Ok.
    let bad = HttpRateSvcProcessor::build_rate_layer(RateConfig {
        tokens_per_second: 0,
        burst_capacity: 10,
        per_host: false,
    });
    assert!(
        bad.is_err(),
        "build_rate_layer must reject a zero token rate"
    );
}
