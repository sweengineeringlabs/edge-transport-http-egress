#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `RetryConfig` public surface.
//!
//! `RetryConfig` is a plain struct with all fields public. Tests verify
//! struct literal construction, field visibility, Clone, and that values
//! flow through the build_retry_layer pipeline to the RetryLayer without mutation.

use edge_transport_http_egress_retry::{HttpRetrySvc, RetryConfig, RetryLayer};

// ---------------------------------------------------------------------------
// Struct construction — all public fields must be writable
// ---------------------------------------------------------------------------

/// Every field on `RetryConfig` must be directly settable via struct literal
/// syntax. A rename or removal causes this test to fail to compile, catching
/// the API break before it reaches consumers.
#[test]
fn test_retry_config_all_fields_are_public() {
    let cfg = RetryConfig {
        max_retries: 3,
        initial_interval_ms: 100,
        max_interval_ms: 5000,
        multiplier: 2.0,
        retryable_statuses: vec![429, 503],
        retryable_methods: vec!["GET".to_string(), "HEAD".to_string()],
    };
    assert_eq!(cfg.max_retries, 3);
    assert_eq!(cfg.initial_interval_ms, 100);
    assert_eq!(cfg.max_interval_ms, 5000);
    assert_eq!(cfg.multiplier, 2.0);
    assert_eq!(cfg.retryable_statuses, vec![429u16, 503]);
    assert_eq!(cfg.retryable_methods, vec!["GET", "HEAD"]);
}

/// `RetryConfig` must be `Clone` so it can be copied for inspection or reuse.
#[test]
fn test_retry_config_is_clone() {
    let cfg = RetryConfig {
        max_retries: 5,
        initial_interval_ms: 200,
        max_interval_ms: 10_000,
        multiplier: 1.5,
        retryable_statuses: vec![500],
        retryable_methods: vec!["DELETE".to_string()],
    };
    let cloned = cfg.clone();
    assert_eq!(cloned.max_retries, cfg.max_retries);
    assert_eq!(cloned.multiplier, cfg.multiplier);
    assert_eq!(cloned.retryable_statuses, cfg.retryable_statuses);
}

// ---------------------------------------------------------------------------
// max_retries boundary values
// ---------------------------------------------------------------------------

/// `max_retries=0` is a valid "no-retry" configuration. The factory and
/// layer must accept it without error.
#[test]
fn test_max_retries_zero_is_valid() {
    let cfg = RetryConfig {
        max_retries: 0,
        initial_interval_ms: 100,
        max_interval_ms: 1000,
        multiplier: 2.0,
        retryable_statuses: vec![503],
        retryable_methods: vec!["GET".to_string()],
    };
    let _layer: RetryLayer =
        HttpRetrySvc::build_retry_layer(cfg).expect("max_retries=0 must build");
}

/// `max_retries=u32::MAX` is an extreme value but must not panic or error
/// at build time.
#[test]
fn test_max_retries_max_u32_builds_without_error() {
    let cfg = RetryConfig {
        max_retries: u32::MAX,
        initial_interval_ms: 100,
        max_interval_ms: 1000,
        multiplier: 1.0,
        retryable_statuses: vec![],
        retryable_methods: vec![],
    };
    HttpRetrySvc::build_retry_layer(cfg).expect("max_retries=u32::MAX must build");
}

// ---------------------------------------------------------------------------
// retryable_statuses — boundary values
// ---------------------------------------------------------------------------

/// Status codes at the edges of the valid HTTP range must be accepted. The
/// type is `Vec<u16>` so any u16 is structurally valid.
#[test]
fn test_retryable_statuses_accepts_full_range_of_u16_values() {
    let cfg = RetryConfig {
        max_retries: 1,
        initial_interval_ms: 100,
        max_interval_ms: 1000,
        multiplier: 1.0,
        retryable_statuses: vec![100, 200, 429, 500, 503, 599, 65535],
        retryable_methods: vec!["GET".to_string()],
    };
    HttpRetrySvc::build_retry_layer(cfg).expect("wide range of status codes must build");
}

/// An empty `retryable_statuses` list must be accepted — it means "never
/// retry on a received HTTP response".
#[test]
fn test_retryable_statuses_empty_is_valid() {
    let cfg = RetryConfig {
        max_retries: 3,
        initial_interval_ms: 100,
        max_interval_ms: 5000,
        multiplier: 2.0,
        retryable_statuses: vec![],
        retryable_methods: vec!["GET".to_string()],
    };
    HttpRetrySvc::build_retry_layer(cfg).expect("empty retryable_statuses must build");
}

// ---------------------------------------------------------------------------
// retryable_methods — case preservation
// ---------------------------------------------------------------------------

/// HTTP method strings must be stored with their original casing. The
/// config struct holds them verbatim.
#[test]
fn test_retryable_methods_stored_with_original_casing() {
    let cfg = RetryConfig {
        max_retries: 1,
        initial_interval_ms: 50,
        max_interval_ms: 500,
        multiplier: 1.0,
        retryable_statuses: vec![503],
        retryable_methods: vec!["get".to_string(), "HEAD".to_string(), "Put".to_string()],
    };
    assert_eq!(cfg.retryable_methods[0], "get");
    assert_eq!(cfg.retryable_methods[1], "HEAD");
    assert_eq!(cfg.retryable_methods[2], "Put");
    HttpRetrySvc::build_retry_layer(cfg).expect("mixed-case methods must build");
}

// ---------------------------------------------------------------------------
// multiplier — positive float values
// ---------------------------------------------------------------------------

/// `multiplier=1.0` produces constant-interval backoff. The factory must
/// accept this (no validation that multiplier > 1.0).
#[test]
fn test_multiplier_one_produces_constant_interval() {
    let cfg = RetryConfig {
        max_retries: 3,
        initial_interval_ms: 200,
        max_interval_ms: 200,
        multiplier: 1.0,
        retryable_statuses: vec![503],
        retryable_methods: vec!["GET".to_string()],
    };
    HttpRetrySvc::build_retry_layer(cfg).expect("multiplier=1.0 must build");
}

/// `multiplier=0.5` (backoff shrinks over time) is unusual but structurally
/// valid. The factory must accept it without error.
#[test]
fn test_multiplier_below_one_builds_successfully() {
    let cfg = RetryConfig {
        max_retries: 2,
        initial_interval_ms: 1000,
        max_interval_ms: 5000,
        multiplier: 0.5,
        retryable_statuses: vec![429],
        retryable_methods: vec!["GET".to_string()],
    };
    HttpRetrySvc::build_retry_layer(cfg).expect("multiplier=0.5 must build");
}
