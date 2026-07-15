//! Integration tests for `HttpCacheSvc::build_cache_layer`.

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvc};

/// @covers: build_cache_layer
#[test]
fn test_build_cache_layer_with_default_config_succeeds() {
    let result = HttpCacheSvc::build_cache_layer(CacheConfig::default());
    assert!(
        result.is_ok(),
        "build_cache_layer with default config must succeed"
    );
}
