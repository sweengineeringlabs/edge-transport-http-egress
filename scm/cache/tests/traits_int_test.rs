//! Integration tests for `api/traits.rs`.
//!
//! `traits.rs` exposes `HttpCacheTrait` as a `pub(crate)` type alias for
//! `dyn HttpCache`.  From outside the crate this is not directly accessible,
//! but we can verify its downstream effect: `MiddlewareHttpCache` must satisfy all bounds
//! required to be stored behind a trait object of that form (`Send + Sync`).
//!
//! These are compile-time proofs — a runtime assertion would add noise without
//! adding information.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::MiddlewareHttpCache;

/// `MiddlewareHttpCache` must be `Send` — required by `HttpCacheTrait = dyn HttpCache`
/// which has `HttpCache: Send + Sync` as supertraits.
#[test]
fn test_cache_layer_satisfies_send_required_by_http_cache_trait() {
    use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor};
    let cfg = CacheConfig {
        default_ttl_seconds: 24,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    };
    // Moving the layer onto a worker thread requires `Send`; asserting on the
    // payload proves the moved value survived intact.
    let layer = HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed");
    let dbg = std::thread::spawn(move || format!("{layer:?}"))
        .join()
        .expect("worker thread must not panic");
    assert!(
        dbg.contains("24"),
        "layer moved across a thread must retain ttl=24; got: {dbg}"
    );
}

/// `MiddlewareHttpCache` must be `Sync` — required by `HttpCacheTrait = dyn HttpCache`.
#[test]
fn test_cache_layer_satisfies_sync_required_by_http_cache_trait() {
    use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor};
    use std::sync::Arc;
    let cfg = CacheConfig {
        default_ttl_seconds: 42,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    };
    // A shared reference held across threads exercises `Sync`: two worker
    // threads read the same `&MiddlewareHttpCache` concurrently. The assertion proves
    // both observed the same, correct configuration.
    let layer =
        Arc::new(HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed"));
    let a = Arc::clone(&layer);
    let b = Arc::clone(&layer);
    let ha = std::thread::spawn(move || format!("{a:?}"));
    let hb = std::thread::spawn(move || format!("{b:?}"));
    let da = ha.join().expect("thread a");
    let db = hb.join().expect("thread b");
    assert_eq!(
        da, db,
        "concurrent readers must observe identical Debug output"
    );
    assert!(
        da.contains("42"),
        "shared layer must retain ttl=42; got: {da}"
    );
}

/// `MiddlewareHttpCache` can be wrapped in a `Box<dyn ... + Send + Sync>` — proof that
/// the trait-object coercion the `traits.rs` alias models is possible.
#[test]
fn test_cache_layer_coercible_to_boxed_send_sync() {
    use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor};
    let cfg = CacheConfig {
        default_ttl_seconds: 5,
        max_entries: 10,
        respect_cache_control: false,
        cache_private: false,
    };
    let layer: MiddlewareHttpCache =
        HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed");
    // Coerce to a boxed `Send + Sync` object — this fails to compile if either
    // bound is absent.
    let _boxed: Box<dyn Send + Sync> = Box::new(layer);
}
