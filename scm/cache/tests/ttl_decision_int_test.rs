//! Integration tests for `TtlDecision`'s api/ and core/ structural
//! counterparts (`api/cache/layer/ttl_decision.rs`,
//! `core/cache/layer/ttl_decision.rs`).
//!
//! `TtlDecision` itself is `pub(crate)` and has its own inline unit tests.
//! From outside the crate we verify the externally-observable effect of a
//! TTL decision: `HttpCache::default_ttl` reflecting the configured fallback
//! TTL used whenever a response carries no `Cache-Control: max-age`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{
    CacheConfig, FallbackTtlRequest, HttpCache, HttpCacheSvcProcessor,
};

/// @covers: default_ttl
#[test]
fn test_default_ttl_large_configured_value_round_trips() {
    let cfg = CacheConfig {
        default_ttl_seconds: 86_400,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    };
    let layer = HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed");
    let resp = layer.default_ttl(FallbackTtlRequest).expect("infallible");
    assert_eq!(resp.seconds, 86_400);
}

/// @covers: default_ttl
#[test]
fn test_default_ttl_independent_across_distinct_layers() {
    let short = HttpCacheSvcProcessor::build_cache_layer(CacheConfig {
        default_ttl_seconds: 5,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    })
    .expect("build must succeed");
    let long = HttpCacheSvcProcessor::build_cache_layer(CacheConfig {
        default_ttl_seconds: 500,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    })
    .expect("build must succeed");
    assert_eq!(
        short
            .default_ttl(FallbackTtlRequest)
            .expect("infallible")
            .seconds,
        5
    );
    assert_eq!(
        long.default_ttl(FallbackTtlRequest)
            .expect("infallible")
            .seconds,
        500
    );
}
