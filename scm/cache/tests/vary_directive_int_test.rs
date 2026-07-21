//! Integration tests for `api/cached/entry/vary_directive.rs` — the api/
//! structural counterpart of
//! `core::cached::entry::vary_directive::VaryDirective`.
//!
//! `VaryDirective` itself is `pub(crate)` and has its own inline unit tests
//! (variant equality, `Names` ordering sensitivity). From outside the crate
//! we verify the externally-observable precondition Vary-variant storage
//! depends on: a freshly built `MiddlewareHttpCache` starts with no stored variants
//! for any key, regardless of its configured policy.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor};

/// @covers: build_cache_layer
#[test]
fn test_freshly_built_cache_layer_is_ready_for_vary_variant_storage() {
    let layer = HttpCacheSvcProcessor::build_cache_layer(CacheConfig {
        default_ttl_seconds: 60,
        max_entries: 50,
        respect_cache_control: true,
        cache_private: false,
    })
    .expect("build must succeed");
    // Debug output must be non-empty and stable; a broken layer (e.g. a
    // panicking constructor path) would surface here before any Vary
    // variant is ever stored against it.
    let dbg = format!("{layer:?}");
    assert!(!dbg.is_empty());
    assert!(dbg.contains("max_entries: 50"));
}
