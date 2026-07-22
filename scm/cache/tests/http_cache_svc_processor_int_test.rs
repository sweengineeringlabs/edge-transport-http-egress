//! Integration tests for `HttpCacheSvcProcessor::build_cache_layer`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor};

/// @covers: build_cache_layer
#[test]
fn test_build_cache_layer_with_default_config_succeeds() {
    let layer = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default())
        .expect("build_cache_layer with default config must succeed");
    let dbg = format!("{layer:?}");
    // The default policy values must actually reach the layer.
    assert!(
        dbg.contains("300"),
        "default ttl 300 must reach the layer; got: {dbg}"
    );
    assert!(
        dbg.contains("10000"),
        "default max_entries 10000 must reach the layer; got: {dbg}"
    );
    // Non-default cross-check: a custom TTL must flow through too, proving the
    // builder honours its input rather than hardcoding the defaults.
    let custom = CacheConfig {
        default_ttl_seconds: 4242,
        max_entries: 7,
        respect_cache_control: false,
        cache_private: true,
    };
    let custom_dbg = format!(
        "{:?}",
        HttpCacheSvcProcessor::build_cache_layer(custom).expect("custom build must succeed")
    );
    assert!(
        custom_dbg.contains("4242"),
        "custom ttl 4242 must reach the layer; got: {custom_dbg}"
    );
}
