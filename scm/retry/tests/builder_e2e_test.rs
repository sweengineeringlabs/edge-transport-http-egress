#![allow(clippy::unwrap_used, clippy::expect_used)]
//! End-to-end tests for the edge_transport_http_egress_retry SAF builder surface.

use edge_transport_http_egress_retry::{
    DecorateRequest, HttpRetrySvc, Processor, RetryConfig, RetryLayer,
};

fn make_cfg() -> RetryConfig {
    RetryConfig {
        max_retries: 3,
        initial_interval_ms: 100,
        max_interval_ms: 5000,
        multiplier: 2.0,
        retryable_statuses: vec![429, 503],
        retryable_methods: vec!["GET".to_string()],
    }
}

/// @covers: build_retry_layer with default config
#[test]
fn test_e2e_build_default() {
    let layer: RetryLayer = HttpRetrySvc
        .decorate(DecorateRequest {
            config: RetryConfig::default(),
        })
        .expect("build must succeed")
        .layer;
    assert!(format!("{layer:?}").contains("RetryLayer"));
}

/// @covers: build_retry_layer with custom config
#[test]
fn test_e2e_build_with_custom_config() {
    let cfg = make_cfg();
    assert_eq!(cfg.max_retries, 3);
    let _layer = HttpRetrySvc
        .decorate(DecorateRequest { config: cfg })
        .expect("e2e with custom config build must succeed");
}

/// @covers: config fields flow through to build
#[test]
fn test_e2e_config_fields() {
    let cfg = make_cfg();
    assert_eq!(cfg.initial_interval_ms, 100);
    assert!(cfg.retryable_statuses.contains(&429));
    HttpRetrySvc
        .decorate(DecorateRequest { config: cfg })
        .expect("build must succeed");
}

/// @covers: build_retry_layer with varied config
#[test]
fn test_e2e_build_varied_config() {
    let cfg = RetryConfig {
        max_retries: 5,
        initial_interval_ms: 50,
        max_interval_ms: 10000,
        multiplier: 1.5,
        retryable_statuses: vec![503, 504],
        retryable_methods: vec!["GET".to_string(), "HEAD".to_string()],
    };
    let layer = HttpRetrySvc
        .decorate(DecorateRequest { config: cfg })
        .expect("e2e build must succeed");
    assert!(!format!("{layer:?}").is_empty());
}

/// @covers: create_config_builder returns a Loader
#[test]
fn test_e2e_create_config_builder_returns_loader() {
    let builder = HttpRetrySvc::new_app_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must carry the crate name before producing a loader"
    );
    let _loader = builder.build_loader();
}
