//! Integration tests for `BreakerLayerBreakerMetrics`'s usability as constructed by the
//! public builder API.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, Loader as _};
use edge_transport_http_egress_breaker::{
    BreakerConfig, BreakerLayerBreakerMetrics, BreakerMetrics, DescribeRequest,
    FailureThresholdRequest, HttpBreakerSvcProcessor, ProcessorFactory,
};

// ---------------------------------------------------------------------------
// create_config_builder / BreakerMetrics / ProcessorFactory::create
// ---------------------------------------------------------------------------

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_loads_shipped_application_config_happy() {
    let config: BreakerConfig = HttpBreakerSvcProcessor::create_config_builder()
        .with_config_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/config"))
        .build_loader()
        .expect("builder must produce a loader")
        .load_section("breaker")
        .expect("shipped breaker configuration must load");

    assert_eq!(config.failure_threshold, 5);
    assert_eq!(config.half_open_after_seconds, 30);
    assert_eq!(config.reset_after_successes, 3);
    assert_eq!(config.failure_statuses, vec![500, 502, 503, 504]);
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_rejects_missing_application_config_error() {
    let loader = HttpBreakerSvcProcessor::create_config_builder()
        .build_loader()
        .expect("builder must produce a loader");
    let result: Result<BreakerConfig, _> = loader.load_section("breaker");
    let error = result.expect_err("missing configuration must not silently use defaults");
    assert!(!error.to_string().is_empty());
}

/// @covers: create
#[test]
fn test_processor_factory_create_produces_a_working_processor() {
    let processor = ProcessorFactory::create();
    let resp = processor.describe(DescribeRequest).expect("infallible");
    assert_eq!(resp.value, "http-breaker");
}

// ---------------------------------------------------------------------------
// Constructed layer is usable
// ---------------------------------------------------------------------------

/// @covers: build_breaker_layer
/// A `BreakerLayerBreakerMetrics` produced by the builder must be ready to use — confirmed
/// by building and formatting it without panic.
#[test]
fn test_breaker_layer_built_from_builder_is_usable() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 10,
        reset_after_successes: 2,
        failure_statuses: vec![500, 503],
    };
    let layer: BreakerLayerBreakerMetrics =
        HttpBreakerSvcProcessor::build_breaker_layer(cfg).expect("build() must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        !dbg.is_empty(),
        "BreakerLayerBreakerMetrics Debug must produce non-empty output"
    );
}

// ---------------------------------------------------------------------------
// Arc<BreakerLayerBreakerMetrics> is usable from another thread
// ---------------------------------------------------------------------------

/// `Arc<BreakerLayerBreakerMetrics>` — the shape reqwest-middleware wraps middleware in
/// internally — must be genuinely shareable across a real thread boundary,
/// not just satisfy a compile-time bound.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_arc_breaker_layer_is_usable_from_another_thread() {
    let layer = std::sync::Arc::new(
        HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default())
            .expect("build() must succeed"),
    );
    let shared = std::sync::Arc::clone(&layer);
    let dbg_on_other_thread = tokio::spawn(async move { format!("{shared:?}") })
        .await
        .expect("spawned task must not panic");
    assert!(
        dbg_on_other_thread.contains("BreakerLayerBreakerMetrics"),
        "Arc-shared layer used on another thread must produce real Debug output: {dbg_on_other_thread}"
    );
}
/// @covers: BreakerMetrics::failure_threshold
#[test]
fn test_failure_threshold_reads_configured_value() {
    let cfg = BreakerConfig {
        failure_threshold: 9,
        ..BreakerConfig::default()
    };
    let layer = HttpBreakerSvcProcessor::build_breaker_layer(cfg).expect("build ok");
    let threshold = layer
        .failure_threshold(FailureThresholdRequest)
        .expect("reading the configured threshold must succeed");
    assert_eq!(threshold.value, 9);
}
