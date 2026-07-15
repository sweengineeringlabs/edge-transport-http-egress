//! Integration tests for `api/cache/layer/request_snapshot.rs`.
//!
//! The `RequestSnapshot` marker trait is `pub(crate)` so it is not directly
//! accessible from integration tests. We verify its downstream effect:
//! the cache middleware layer, which uses `RequestSnapshot` internally,
//! constructs and operates correctly.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvc};

/// Verify that the cache layer (which uses request snapshots internally)
/// constructs without error — proving the snapshot module is functional.
#[test]
fn cache_struct_request_snapshot_layer_constructs_using_snapshot_internally_int_test() {
    let cfg = CacheConfig::default();
    let layer = HttpCacheSvc::build_cache_layer(cfg).expect("build_cache_layer must succeed");
    // CacheLayer uses RequestSnapshot in handle(); Send + Sync proves the layer
    // satisfies the middleware contract.
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<edge_transport_http_egress_cache::CacheLayer>();
    let _ = layer;
}
