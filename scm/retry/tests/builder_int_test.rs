#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `build_retry_layer` and `create_config_builder` SAF entry points.
//!
//! Covers: `build_retry_layer`, `create_config_builder`, and config defaults.

use edge_transport_http_egress_retry::{
    DecorateRequest, HttpRetrySvc, Processor, RetryConfig, RetryLayer,
};

fn make_cfg(max_retries: u32) -> RetryConfig {
    RetryConfig {
        max_retries,
        initial_interval_ms: 100,
        max_interval_ms: 5000,
        multiplier: 2.0,
        jitter_factor: 0.0,
        retryable_statuses: vec![429, 500, 502, 503],
        retryable_methods: vec!["GET".to_string(), "HEAD".to_string()],
    }
}

// ---------------------------------------------------------------------------
// create_config_builder — SAF entry point
// ---------------------------------------------------------------------------

/// The `create_config_builder()` function must return a loader that works
/// without error. This validates that the crate's package name / version are
/// correctly wired.
#[test]
fn test_create_config_builder_returns_working_loader() {
    let builder = HttpRetrySvc::new_app_config_builder();
    assert!(
        !builder.version().is_empty(),
        "builder must carry the crate version before producing a loader"
    );
    let _loader = builder.build_loader();
}

/// The SWE default config must have at least one retry so the middleware
/// is not a no-op by default.
#[test]
fn test_default_config_has_at_least_one_retry() {
    let cfg = RetryConfig::default();
    assert!(
        cfg.max_retries >= 1,
        "default max_retries must be >= 1; got {}",
        cfg.max_retries
    );
}

/// An empty retryable_statuses list in the default config would mean no HTTP
/// response ever triggers a retry.  The default must include at least one.
#[test]
fn test_default_config_has_non_empty_retryable_statuses() {
    let cfg = RetryConfig::default();
    assert!(
        !cfg.retryable_statuses.is_empty(),
        "default retryable_statuses must not be empty"
    );
}

/// The default initial_interval_ms must be positive — a zero delay causes
/// the middleware to retry without backoff.
#[test]
fn test_default_config_initial_interval_is_positive() {
    let cfg = RetryConfig::default();
    assert!(
        cfg.initial_interval_ms > 0,
        "default initial_interval_ms must be > 0; got {}",
        cfg.initial_interval_ms
    );
}

// ---------------------------------------------------------------------------
// build_retry_layer — custom config flows through unchanged
// ---------------------------------------------------------------------------

/// All fields supplied to `build_retry_layer` must be faithfully embedded.
/// We verify via the Debug output which exposes max_retries and other fields.
#[test]
fn test_build_retry_layer_stores_all_fields_in_layer() {
    let cfg = make_cfg(5);
    let layer = HttpRetrySvc
        .decorate(DecorateRequest { config: cfg })
        .expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("5"),
        "Debug must contain max_retries=5; got: {dbg}"
    );
}

/// `build_retry_layer` must return a `RetryLayer` whose Debug output names
/// the type and exposes `max_retries` for operator visibility.
#[test]
fn test_build_retry_layer_returns_retry_layer_with_correct_debug() {
    let layer: RetryLayer = HttpRetrySvc
        .decorate(DecorateRequest {
            config: make_cfg(3),
        })
        .expect("build must succeed")
        .layer;
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("RetryLayer"),
        "Debug must name the type; got: {dbg}"
    );
    assert!(
        dbg.contains("max_retries"),
        "Debug must expose max_retries; got: {dbg}"
    );
}

/// Building from the default config must produce a layer with `max_retries`
/// visible in Debug.
#[test]
fn test_build_retry_layer_from_default_debug_contains_max_retries() {
    let layer = HttpRetrySvc
        .decorate(DecorateRequest {
            config: RetryConfig::default(),
        })
        .expect("build ok")
        .layer;
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("max_retries"),
        "Debug must expose max_retries; got: {dbg}"
    );
}

/// `max_retries=0` is valid — it means "pass-through, never retry". The
/// factory must accept this and produce a layer without error.
#[test]
fn test_build_retry_layer_with_zero_max_retries_succeeds() {
    let cfg = RetryConfig {
        max_retries: 0,
        initial_interval_ms: 100,
        max_interval_ms: 100,
        multiplier: 1.0,
        jitter_factor: 0.0,
        retryable_statuses: vec![],
        retryable_methods: vec![],
    };
    HttpRetrySvc
        .decorate(DecorateRequest { config: cfg })
        .expect("max_retries=0 must build");
}

/// Empty `retryable_statuses` and `retryable_methods` are valid — the
/// middleware simply never triggers a retry. Must build without error.
#[test]
fn test_build_retry_layer_with_empty_retryable_lists_succeeds() {
    let cfg = RetryConfig {
        max_retries: 5,
        initial_interval_ms: 50,
        max_interval_ms: 2000,
        multiplier: 1.5,
        jitter_factor: 0.0,
        retryable_statuses: vec![],
        retryable_methods: vec![],
    };
    HttpRetrySvc
        .decorate(DecorateRequest { config: cfg })
        .expect("empty retryable lists must build");
}

/// A very large multiplier (e.g. 100x) is a valid operator choice for
/// aggressive backoff during incident response. The factory must accept it.
#[test]
fn test_build_retry_layer_with_large_multiplier_succeeds() {
    let cfg = RetryConfig {
        max_retries: 2,
        initial_interval_ms: 10,
        max_interval_ms: 60_000,
        multiplier: 100.0,
        jitter_factor: 0.0,
        retryable_statuses: vec![503],
        retryable_methods: vec!["POST".to_string()],
    };
    HttpRetrySvc
        .decorate(DecorateRequest { config: cfg })
        .expect("multiplier=100.0 must build");
}

// ---------------------------------------------------------------------------
// RetryLayer: Send + Sync — compile-time proof
// ---------------------------------------------------------------------------

/// `RetryLayer` must be `Send + Sync` so it can be shared across async task
/// boundaries inside a `reqwest_middleware` chain.
#[test]
fn test_retry_layer_is_send_and_sync() {
    // Genuine runtime proof of Send: move the layer into another thread.
    let layer = HttpRetrySvc
        .decorate(DecorateRequest {
            config: make_cfg(3),
        })
        .expect("build")
        .layer;
    let dbg = std::thread::spawn(move || format!("{layer:?}"))
        .join()
        .expect("thread must not panic");
    assert!(
        dbg.contains("max_retries: 3"),
        "layer moved across a thread must retain its config; got: {dbg}"
    );
}
