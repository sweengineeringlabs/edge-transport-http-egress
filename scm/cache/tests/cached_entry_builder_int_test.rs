//! Integration tests for `api/cached/entry/cached_entry_builder.rs` — the
//! api/ structural counterpart of
//! `core::cached::entry::cached_entry_builder::CachedEntryBuilder`.
//!
//! `CachedEntryBuilder` itself is `pub(crate)` and has its own inline unit
//! tests. From outside the crate we verify the externally-observable effect:
//! a `MiddlewareHttpCache` built with `cache_private = true` must be constructible and
//! carry that policy through to its Debug output, since entries built via
//! `CachedEntryBuilder` are only ever stored when the layer's policy allows
//! it.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor};

/// @covers: build_cache_layer
#[test]
fn test_cache_layer_with_cache_private_true_is_usable() {
    let cfg = CacheConfig {
        default_ttl_seconds: 60,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: true,
    };
    let layer = HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed");
    assert!(format!("{layer:?}").contains("cache_private: true"));
}

/// @covers: build_cache_layer
#[test]
fn test_cache_layer_with_cache_private_false_is_usable() {
    let cfg = CacheConfig {
        default_ttl_seconds: 60,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    };
    let layer = HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed");
    assert!(format!("{layer:?}").contains("cache_private: false"));
}
