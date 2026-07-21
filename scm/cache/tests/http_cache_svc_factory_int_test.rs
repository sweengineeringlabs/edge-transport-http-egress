//! Integration tests for [`HttpCacheSvcFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{
    CacheConfig, FallbackTtlRequest, HttpCacheSvcFactory, HttpCacheSvcProcessor,
};

/// @covers: from_layer
#[test]
fn test_from_layer_upcasts_to_metrics_happy() {
    let config = CacheConfig {
        default_ttl_seconds: 900,
        max_entries: 100,
        respect_cache_control: true,
        cache_private: false,
    };
    let layer = HttpCacheSvcProcessor::build_cache_layer(config).expect("build ok");
    let cache = HttpCacheSvcFactory::from_layer(layer);
    let resp = cache
        .default_ttl(FallbackTtlRequest)
        .expect("upcast must genuinely work");
    assert_eq!(resp.seconds, 900);
}

/// @covers: from_layer
#[test]
fn test_from_layer_reflects_a_non_default_config_value_edge() {
    let config = CacheConfig {
        default_ttl_seconds: 42,
        ..CacheConfig::default()
    };
    let layer = HttpCacheSvcProcessor::build_cache_layer(config).expect("build ok");
    let cache = HttpCacheSvcFactory::from_layer(layer);
    let resp = cache.default_ttl(FallbackTtlRequest).expect("must succeed");
    assert_ne!(resp.seconds, 0);
    assert_eq!(resp.seconds, 42);
}

/// @covers: from_layer
#[test]
fn test_from_layer_matches_default_config_edge() {
    let layer = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default()).expect("build ok");
    let cache = HttpCacheSvcFactory::from_layer(layer);
    let resp = cache.default_ttl(FallbackTtlRequest).expect("must succeed");
    assert_eq!(resp.seconds, CacheConfig::default().default_ttl_seconds);
}
