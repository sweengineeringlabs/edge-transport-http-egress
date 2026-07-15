//! Integration tests for `api/cached/entry/cache_entry_helper.rs`.
//!
//! `CacheEntryHelper` is `pub(crate)`. We verify its downstream effect:
//! the cache layer, which delegates to `CacheEntryHelper` for vary matching
//! and revalidation logic, correctly reports Send + Sync bounds.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvc};

/// The cache layer depends on CacheEntryHelper internally for Vary and
/// revalidation decisions. Verify the layer is operational.
#[test]
fn cache_struct_entry_helper_layer_is_send_sync_indicating_helper_is_crate_safe_int_test() {
    let cfg = CacheConfig::default();
    let _layer = HttpCacheSvc::build_cache_layer(cfg).expect("build must succeed");
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<edge_transport_http_egress_cache::CacheLayer>();
}
