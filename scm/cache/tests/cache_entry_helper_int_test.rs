//! Integration tests for `api/cached/entry/cache_entry_helper.rs` — the api/
//! structural counterpart of
//! `core::cached::entry::cache_entry_helper::CacheEntryHelper`.
//!
//! `CacheEntryHelper` itself is `pub(crate)` and has its own inline unit
//! tests (max-age parsing, stale-while-revalidate parsing, Vary matching).
//! From outside the crate we verify the externally-observable effect:
//! `HttpCache::default_ttl` behaves correctly when `respect_cache_control` is
//! toggled, since that flag gates whether `CacheEntryHelper`'s
//! Cache-Control parsing runs at all.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{
    CacheConfig, FallbackTtlRequest, HttpCache, HttpCacheSvcProcessor,
};

/// @covers: default_ttl
#[test]
fn test_default_ttl_unaffected_by_respect_cache_control_flag_happy() {
    let with_respect = HttpCacheSvcProcessor::build_cache_layer(CacheConfig {
        default_ttl_seconds: 120,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    })
    .expect("build must succeed");
    let without_respect = HttpCacheSvcProcessor::build_cache_layer(CacheConfig {
        default_ttl_seconds: 120,
        max_entries: 10,
        respect_cache_control: false,
        cache_private: false,
    })
    .expect("build must succeed");
    assert_eq!(
        with_respect
            .default_ttl(FallbackTtlRequest)
            .expect("infallible")
            .seconds,
        without_respect
            .default_ttl(FallbackTtlRequest)
            .expect("infallible")
            .seconds,
        "default_ttl reports the configured fallback regardless of respect_cache_control, \
         which only governs upstream Cache-Control parsing during request handling"
    );
}

/// @covers: default_ttl
#[test]
fn test_default_ttl_zero_seconds_unaffected_by_respect_cache_control_flag_edge() {
    let with_respect = HttpCacheSvcProcessor::build_cache_layer(CacheConfig {
        default_ttl_seconds: 0,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    })
    .expect("build must succeed");
    let without_respect = HttpCacheSvcProcessor::build_cache_layer(CacheConfig {
        default_ttl_seconds: 0,
        max_entries: 10,
        respect_cache_control: false,
        cache_private: false,
    })
    .expect("build must succeed");
    assert_eq!(
        with_respect
            .default_ttl(FallbackTtlRequest)
            .expect("infallible")
            .seconds,
        0
    );
    assert_eq!(
        without_respect
            .default_ttl(FallbackTtlRequest)
            .expect("infallible")
            .seconds,
        0,
        "a zero fallback TTL must round-trip as zero regardless of respect_cache_control"
    );
}
