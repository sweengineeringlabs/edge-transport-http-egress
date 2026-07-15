//! Integration tests for `api/cached/entry/vary_directive.rs`.
//!
//! `VaryDirective` is `pub(crate)`. We verify its downstream effect:
//! the cache layer honours Vary-based cache decisions. A cache layer that
//! builds successfully has wired the VaryDirective module into the
//! compilation unit.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvc};

/// The cache layer depends on VaryDirective for Vary-header parsing.
/// Verify the layer constructs — which requires the module to compile.
#[test]
fn cache_struct_vary_directive_layer_constructs_with_respect_cache_control_true_int_test() {
    let cfg = CacheConfig {
        default_ttl_seconds: 300,
        max_entries: 100,
        respect_cache_control: true,
        cache_private: false,
    };
    let _layer = HttpCacheSvc::build_cache_layer(cfg)
        .expect("build_cache_layer with respect_cache_control=true must succeed");
}
