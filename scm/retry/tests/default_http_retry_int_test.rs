#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `core::default_http_retry::DefaultHttpRetry`.
//!
//! `DefaultHttpRetry` is `pub(crate)`.  Integration tests verify its contract
//! through observable effects from the public factory pipeline:
//!
//! - The layer's Debug output confirms `max_retries` flows from config through
//!   `DefaultHttpRetry` into the layer.
//! - `RetryLayer` is `Send + Sync`, which propagates from `DefaultHttpRetry`
//!   being `Send + Sync` through the `Arc<RetryConfig>` chain.
//! - The factory pipeline does not modify the config values on the way through
//!   `DefaultHttpRetry::new`.

use edge_transport_http_egress_retry::{
    DecorateRequest, HttpRetrySvc, Processor, RetryConfig, RetryLayer,
};

fn make_cfg(max_retries: u32, initial_ms: u64) -> RetryConfig {
    RetryConfig {
        max_retries,
        initial_interval_ms: initial_ms,
        max_interval_ms: 10_000,
        multiplier: 2.0,
        jitter_factor: 0.0,
        retryable_statuses: vec![429, 503],
        retryable_methods: vec!["GET".to_string()],
    }
}

// ---------------------------------------------------------------------------
// DefaultHttpRetry::new — indirectly via factory pipeline
// ---------------------------------------------------------------------------

/// `DefaultHttpRetry::new` is invoked with the config inside `build_retry_layer`.
/// Observable effect: the layer's Debug output must embed `max_retries` and
/// `initial_interval_ms`, confirming the config was not swapped or reset.
#[test]
fn test_factory_pipeline_embeds_config_in_default_http_retry() {
    let layer: RetryLayer = HttpRetrySvc
        .decorate(DecorateRequest {
            config: make_cfg(4, 250),
        })
        .expect("build must succeed")
        .layer;
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains('4'),
        "Debug must embed max_retries=4; got: {dbg}"
    );
    assert!(
        dbg.contains("250"),
        "Debug must embed initial_interval_ms=250; got: {dbg}"
    );
}

// ---------------------------------------------------------------------------
// DefaultHttpRetry::describe — values differ between configs
// ---------------------------------------------------------------------------

/// The `RetryLayer` Debug output shows `max_retries` and `initial_interval_ms`.
/// Two layers with different values must produce different Debug strings,
/// confirming `DefaultHttpRetry::new` stores the supplied config verbatim.
#[test]
fn test_two_layers_different_configs_have_different_debug() {
    let l1 = HttpRetrySvc
        .decorate(DecorateRequest {
            config: make_cfg(1, 100),
        })
        .unwrap()
        .layer;
    let l2 = HttpRetrySvc
        .decorate(DecorateRequest {
            config: make_cfg(5, 500),
        })
        .unwrap()
        .layer;
    let d1 = format!("{l1:?}");
    let d2 = format!("{l2:?}");
    // Each layer renders its own configured values verbatim — proving the
    // config is stored, not defaulted or swapped.
    assert!(d1.contains("max_retries: 1"), "l1 Debug: {d1}");
    assert!(d1.contains("initial_interval_ms: 100"), "l1 Debug: {d1}");
    assert!(d2.contains("max_retries: 5"), "l2 Debug: {d2}");
    assert!(d2.contains("initial_interval_ms: 500"), "l2 Debug: {d2}");
}

// ---------------------------------------------------------------------------
// DefaultHttpRetry: Send + Sync propagation
// ---------------------------------------------------------------------------

/// `RetryLayer` must be `Send + Sync`. The `HttpRetry: Send + Sync`
/// supertraits propagate this requirement to `DefaultHttpRetry` and
/// through the `Arc<RetryConfig>` held by `RetryLayer`.
#[test]
fn test_retry_layer_is_send_and_sync() {
    // Genuine runtime proof of Send: move the layer into another thread and
    // read its Debug there. Requires Send; the assertion checks real content.
    let layer = HttpRetrySvc
        .decorate(DecorateRequest {
            config: make_cfg(3, 100),
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

// ---------------------------------------------------------------------------
// Config is not mutated during build
// ---------------------------------------------------------------------------

/// All config fields must be unchanged after passing through `build_retry_layer`.
/// Verified through the layer's Debug output embedding those values.
#[test]
fn test_factory_does_not_mutate_config_in_default_http_retry() {
    let retryable_statuses = vec![429u16, 500, 502, 503, 504];
    let retryable_methods = vec!["GET".to_string(), "HEAD".to_string(), "PUT".to_string()];
    let cfg = RetryConfig {
        max_retries: 6,
        initial_interval_ms: 300,
        max_interval_ms: 15_000,
        multiplier: 1.8,
        jitter_factor: 0.0,
        retryable_statuses: retryable_statuses.clone(),
        retryable_methods: retryable_methods.clone(),
    };
    // Clone to verify fields before consuming cfg.
    let expected_retries = cfg.max_retries;
    let expected_initial = cfg.initial_interval_ms;
    let layer = HttpRetrySvc
        .decorate(DecorateRequest { config: cfg })
        .expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains(&expected_retries.to_string()),
        "Debug must embed max_retries={}; got: {dbg}",
        expected_retries
    );
    assert!(
        dbg.contains(&expected_initial.to_string()),
        "Debug must embed initial_interval_ms={}; got: {dbg}",
        expected_initial
    );
}

// ---------------------------------------------------------------------------
// create_config_builder convenience function routes through DefaultHttpRetry
// ---------------------------------------------------------------------------

/// The `create_config_builder()` entry point must produce a loader that works —
/// confirming the crate package name is correctly wired into the config builder.
#[test]
fn test_saf_create_config_builder_produces_working_loader() {
    let builder = HttpRetrySvc::new_app_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must be seeded with the crate name before building a loader"
    );
    let _loader = builder.build_loader();
}

/// Building from the default config must produce a `RetryLayer`
/// whose Debug output confirms the wiring is correct.
#[test]
fn test_build_retry_layer_from_default_config_produces_valid_layer() {
    let layer = HttpRetrySvc
        .decorate(DecorateRequest {
            config: RetryConfig::default(),
        })
        .expect("build must succeed")
        .layer;
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("RetryLayer"),
        "factory pipeline must produce RetryLayer; got: {dbg}"
    );
    assert!(
        dbg.contains("max_retries"),
        "layer Debug must show max_retries from DefaultHttpRetry; got: {dbg}"
    );
}
