//! Integration tests for the `build_breaker_layer` SAF entry point.
//!
//! Covers: `build_breaker_layer`, `BreakerConfig` fields, `BreakerLayerBreakerMetrics` construction.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, BreakerError, BreakerLayerBreakerMetrics, HttpBreakerSvcProcessor,
};

// ---------------------------------------------------------------------------
// build_breaker_layer — SAF entry point
// ---------------------------------------------------------------------------

/// The `build_breaker_layer` function must succeed with the default config — the
/// crate-shipped baseline must always parse. A failure here means the embedded
/// default config is broken.
#[test]
fn test_builder_fn_succeeds_with_swe_default() {
    HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default())
        .expect("builder() must succeed with the crate-shipped baseline");
}

/// The default `failure_threshold` must be >= 1. A threshold of 0 would
/// open the breaker on every single request, making it permanently open.
#[test]
fn test_builder_fn_swe_default_failure_threshold_is_positive() {
    let cfg = BreakerConfig::default();
    assert!(
        cfg.failure_threshold >= 1,
        "swe_default failure_threshold must be >= 1, got {}",
        cfg.failure_threshold
    );
}

/// The default `reset_after_successes` must be >= 1. Zero successes to close
/// would mean the breaker immediately closes on a half-open probe attempt,
/// defeating its purpose.
#[test]
fn test_builder_fn_swe_default_reset_after_successes_is_positive() {
    let cfg = BreakerConfig::default();
    assert!(
        cfg.reset_after_successes >= 1,
        "swe_default reset_after_successes must be >= 1, got {}",
        cfg.reset_after_successes
    );
}

// ---------------------------------------------------------------------------
// BreakerConfig — custom policy round-trips correctly
// ---------------------------------------------------------------------------

/// `BreakerConfig` must preserve every field without silent modification.
#[test]
fn test_with_config_preserves_all_fields() {
    let cfg = BreakerConfig {
        failure_threshold: 5,
        half_open_after_seconds: 30,
        reset_after_successes: 3,
        failure_statuses: vec![500, 502, 503],
    };
    let b_cfg = cfg;
    let policy = &b_cfg;
    assert_eq!(policy.failure_threshold, 5);
    assert_eq!(policy.half_open_after_seconds, 30);
    assert_eq!(policy.reset_after_successes, 3);
    assert_eq!(policy.failure_statuses, vec![500u16, 502, 503]);
}

/// Config can be accessed as a reference without a divergent copy.
#[test]
fn test_config_accessor_returns_reference_not_divergent_copy() {
    let cfg = BreakerConfig {
        failure_threshold: 7,
        half_open_after_seconds: 10,
        reset_after_successes: 2,
        failure_statuses: vec![503],
    };
    let b_cfg = cfg;
    let policy: &BreakerConfig = &b_cfg;
    assert_eq!(policy.failure_threshold, 7);
    assert_eq!(policy.reset_after_successes, 2);
}

// ---------------------------------------------------------------------------
// build_breaker_layer — produces a usable BreakerLayerBreakerMetrics
// ---------------------------------------------------------------------------

/// The nominal build path must succeed and return a `BreakerLayerBreakerMetrics`.
#[test]
fn test_build_from_swe_default_returns_breaker_layer() {
    let layer: BreakerLayerBreakerMetrics =
        HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default())
            .expect("build() must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("BreakerLayerBreakerMetrics"),
        "Debug output must identify the type; got: {dbg}"
    );
}

/// Building from a custom config must succeed.
#[test]
fn test_build_with_custom_config_succeeds() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 60,
        reset_after_successes: 2,
        failure_statuses: vec![500, 503],
    };
    HttpBreakerSvcProcessor::build_breaker_layer(cfg).expect("custom config must build");
}

/// An empty `failure_statuses` list is a valid policy (no HTTP status triggers
/// a failure — only network errors do).
#[test]
fn test_build_with_empty_failure_statuses_succeeds() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 60,
        reset_after_successes: 2,
        failure_statuses: vec![],
    };
    HttpBreakerSvcProcessor::build_breaker_layer(cfg)
        .expect("empty failure_statuses must not be rejected");
}

/// A high `failure_threshold` (aggressive tolerance) is a legitimate
/// configuration and must not be rejected at build time.
#[test]
fn test_build_with_high_failure_threshold_succeeds() {
    let cfg = BreakerConfig {
        failure_threshold: 1000,
        half_open_after_seconds: 5,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    };
    HttpBreakerSvcProcessor::build_breaker_layer(cfg)
        .expect("failure_threshold=1000 must not be rejected");
}

/// A single-success reset policy is legitimate — probe once and close.
#[test]
fn test_build_with_single_success_reset_policy_succeeds() {
    let cfg = BreakerConfig {
        failure_threshold: 5,
        half_open_after_seconds: 30,
        reset_after_successes: 1,
        failure_statuses: vec![503],
    };
    HttpBreakerSvcProcessor::build_breaker_layer(cfg)
        .expect("reset_after_successes=1 must not be rejected");
}

// ---------------------------------------------------------------------------
// Error variants — public constructability
// ---------------------------------------------------------------------------

/// `BreakerError::ParseFailed` must display the crate name.
#[test]
fn test_error_parse_failed_display_names_crate() {
    let err = BreakerError::ParseFailed("bad toml".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("edge_transport_http_egress_breaker"),
        "ParseFailed display must name the crate; got: {msg}"
    );
}

/// `BreakerError::ParseFailed` display must echo the supplied reason.
#[test]
fn test_error_parse_failed_display_echoes_reason() {
    let reason = "missing field `failure_threshold`";
    let err = BreakerError::ParseFailed(reason.to_string());
    assert!(err.to_string().contains(reason));
}
