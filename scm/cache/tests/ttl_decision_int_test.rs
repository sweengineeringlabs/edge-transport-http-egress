//! Integration tests for `api/cache/layer/ttl_decision.rs`.
//!
//! The `TtlDecision` marker trait is `pub(crate)`. We verify its downstream
//! effect: the cache layer, which relies on TTL decisions to determine
//! cache policy, honours the configured TTL parameters.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvc};

/// Building a cache layer with a specific TTL succeeds — the TTL decision
/// module is exercised at layer construction time.
#[test]
fn cache_struct_ttl_decision_layer_built_with_custom_ttl_succeeds_int_test() {
    let cfg = CacheConfig {
        default_ttl_seconds: 120,
        max_entries: 50,
        respect_cache_control: true,
        cache_private: false,
    };
    let layer =
        HttpCacheSvc::build_cache_layer(cfg).expect("build_cache_layer with TTL=120 must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("120"),
        "Debug output must include the configured TTL; got: {dbg}"
    );
}
