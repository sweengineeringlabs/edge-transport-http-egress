//! Integration tests for `api/cache/layer/request_snapshot.rs` — the api/
//! structural counterpart of `core::cache::layer::request_snapshot::RequestSnapshot`.
//!
//! `RequestSnapshot` itself is `pub(crate)` (captures method/url/headers for
//! the SWR background refresh) and has its own inline unit tests in core/.
//! From outside the crate we verify the externally-observable effect: a
//! `MiddlewareHttpCache` built from distinct configs must retain the config that
//! downstream request handling (which snapshots the request) depends on.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor};

/// @covers: build_cache_layer
#[test]
fn test_cache_layer_built_with_max_entries_one_is_usable() {
    let cfg = CacheConfig {
        default_ttl_seconds: 30,
        max_entries: 1,
        respect_cache_control: true,
        cache_private: false,
    };
    let layer = HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("max_entries: 1"),
        "a single-entry store must still build and report its capacity; got: {dbg}"
    );
}

/// @covers: build_cache_layer
#[test]
fn test_cache_layer_built_with_large_max_entries_is_usable() {
    let cfg = CacheConfig {
        default_ttl_seconds: 30,
        max_entries: 1_000_000,
        respect_cache_control: true,
        cache_private: false,
    };
    let layer = HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed");
    assert!(format!("{layer:?}").contains("max_entries: 1000000"));
}
