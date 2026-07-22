//! Integration tests for `api/http_cache.rs` — the `HttpCache` trait.
//!
//! The `HttpCache` trait is `pub(crate)` so consumers cannot name it directly.
//! What we CAN observe from outside is that `MiddlewareHttpCache` (the concrete type
//! produced by `HttpCacheSvcProcessor::build_cache_layer(config)`) satisfies the trait's bounds (`Send + Sync`),
//! and that the layer can be passed to any generic context that requires those
//! bounds.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor, MiddlewareHttpCache};
use std::sync::Arc;

fn layer_with_ttl(ttl: u64) -> MiddlewareHttpCache {
    let cfg = CacheConfig {
        default_ttl_seconds: ttl,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    };
    HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed")
}

// ---------------------------------------------------------------------------
// Send + Sync — exercised across real thread boundaries, with payload checks
// ---------------------------------------------------------------------------

/// `MiddlewareHttpCache` must be `Send`: moving one onto a worker thread must compile
/// and the moved value must remain a usable layer that carries its config.
#[test]
fn test_http_cache_bound_send_satisfied_by_cache_layer() {
    let layer = layer_with_ttl(11);
    let dbg = std::thread::spawn(move || format!("{layer:?}"))
        .join()
        .expect("worker thread must not panic");
    assert!(
        dbg.contains("11"),
        "moved layer must retain ttl=11; got: {dbg}"
    );
}

/// `MiddlewareHttpCache` must be `Sync`: a shared `&MiddlewareHttpCache` (behind `Arc`) must be
/// readable from another thread.
#[test]
fn test_http_cache_bound_sync_satisfied_by_cache_layer() {
    let layer = Arc::new(layer_with_ttl(22));
    let shared = Arc::clone(&layer);
    let dbg = std::thread::spawn(move || format!("{shared:?}"))
        .join()
        .expect("worker thread must not panic");
    assert!(
        dbg.contains("22"),
        "shared layer must retain ttl=22 across threads; got: {dbg}"
    );
}

/// Combined `Send + Sync`: two threads read the same shared layer concurrently
/// and must observe identical, correct output.
#[test]
fn test_http_cache_send_and_sync_combined_bound_satisfied() {
    let layer = Arc::new(layer_with_ttl(33));
    let a = Arc::clone(&layer);
    let b = Arc::clone(&layer);
    let ha = std::thread::spawn(move || format!("{a:?}"));
    let hb = std::thread::spawn(move || format!("{b:?}"));
    let da = ha.join().expect("thread a");
    let db = hb.join().expect("thread b");
    assert_eq!(da, db, "concurrent readers must agree");
    assert!(
        da.contains("33"),
        "shared layer must retain ttl=33; got: {da}"
    );
}

// ---------------------------------------------------------------------------
// Layer is usable after being constructed via build_cache_layer
// ---------------------------------------------------------------------------

/// A `MiddlewareHttpCache` produced by `build_cache_layer` must be ready to use —
/// confirmed by successfully building and formatting it.
#[test]
fn test_cache_layer_built_from_builder_is_usable() {
    let cfg = CacheConfig {
        default_ttl_seconds: 30,
        max_entries: 100,
        respect_cache_control: true,
        cache_private: false,
    };
    let layer: MiddlewareHttpCache =
        HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build() must succeed");
    // If MiddlewareHttpCache's HttpCache impl were broken (e.g. panics on construction)
    // this test would surface it.
    let dbg = format!("{layer:?}");
    assert!(
        !dbg.is_empty(),
        "MiddlewareHttpCache Debug must produce non-empty output"
    );
}

// ---------------------------------------------------------------------------
// Arc<MiddlewareHttpCache> is also Send + Sync (shared middleware ownership)
// ---------------------------------------------------------------------------

/// `Arc<MiddlewareHttpCache>` must be `Send + Sync` — reqwest-middleware wraps
/// middleware in `Arc` internally, so this is a practical requirement. Moving
/// the `Arc` to a worker thread exercises that bound; the surviving strong
/// count proves both handles are live.
#[test]
fn test_arc_cache_layer_is_send_and_sync() {
    let layer = Arc::new(layer_with_ttl(44));
    let moved = Arc::clone(&layer);
    let count = std::thread::spawn(move || Arc::strong_count(&moved))
        .join()
        .expect("worker thread must not panic");
    assert_eq!(
        count, 2,
        "both the original and the moved Arc handle must be live across the thread boundary"
    );
    assert!(
        format!("{layer:?}").contains("44"),
        "the retained handle must still address the ttl=44 layer"
    );
}
