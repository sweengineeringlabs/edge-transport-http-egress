//! Integration tests for `FallbackTtlRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{
    CacheConfig, FallbackTtlRequest, HttpCache, HttpCacheSvcProcessor,
};

fn layer_with_ttl(ttl: u64) -> edge_transport_http_egress_cache::MiddlewareHttpCache {
    let cfg = CacheConfig {
        default_ttl_seconds: ttl,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    };
    HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed")
}

/// @covers: default_ttl
#[test]
fn test_default_ttl_configured_seconds_returns_the_same_value_happy() {
    let layer = layer_with_ttl(300);
    let resp = layer.default_ttl(FallbackTtlRequest).expect("infallible");
    assert_eq!(resp.seconds, 300);
}

/// @covers: default_ttl
#[test]
fn test_default_ttl_zero_seconds_is_a_valid_boundary_edge() {
    let layer = layer_with_ttl(0);
    let resp = layer.default_ttl(FallbackTtlRequest).expect("infallible");
    assert_eq!(
        resp.seconds, 0,
        "a zero fallback TTL must round-trip unchanged, not be silently defaulted"
    );
}
