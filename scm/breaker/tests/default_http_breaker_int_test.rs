//! Integration tests for `core/default_http_breaker.rs`.
//!
//! `DefaultHttpBreaker` is `pub(crate)`.  Its observable effect is through the
//! SAF `HttpBreakerSvc::build_breaker_layer()` function, which accepts a `BreakerConfig`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{BreakerConfig, HttpBreakerSvc};

// ---------------------------------------------------------------------------
// SWE baseline — verify default config has production-safe values
// ---------------------------------------------------------------------------

/// `HttpBreakerSvc::build_breaker_layer(BreakerConfig::default())` must succeed without error.
#[test]
fn test_default_http_breaker_swe_default_builder_succeeds() {
    HttpBreakerSvc::build_breaker_layer(BreakerConfig::default())
        .expect("swe_default baseline must parse without error");
}

/// Default `failure_threshold` must be >= 1.  Zero would trip on any request.
#[test]
fn test_default_http_breaker_swe_default_failure_threshold_is_positive() {
    let cfg = BreakerConfig::default();
    assert!(
        cfg.failure_threshold >= 1,
        "swe_default failure_threshold must be >= 1, got {}",
        cfg.failure_threshold
    );
}

/// Default `reset_after_successes` must be >= 1.
#[test]
fn test_default_http_breaker_swe_default_reset_after_successes_is_positive() {
    let cfg = BreakerConfig::default();
    assert!(
        cfg.reset_after_successes >= 1,
        "swe_default reset_after_successes must be >= 1, got {}",
        cfg.reset_after_successes
    );
}

/// Building from the SWE default must produce a valid `BreakerLayer`.
#[test]
fn test_default_http_breaker_swe_default_builds_layer() {
    HttpBreakerSvc::build_breaker_layer(BreakerConfig::default())
        .expect("build from swe_default must succeed");
}

// ---------------------------------------------------------------------------
// Custom config is not overridden by the default-loading path
// ---------------------------------------------------------------------------

/// A consumer-supplied config must pass through unchanged — no field must
/// be silently replaced by a SWE default.
#[test]
fn test_default_http_breaker_custom_config_is_not_overridden_by_swe_default() {
    let custom = BreakerConfig {
        failure_threshold: 99,
        half_open_after_seconds: 7,
        reset_after_successes: 11,
        failure_statuses: vec![418],
    };
    assert_eq!(custom.failure_threshold, 99);
    assert_eq!(custom.half_open_after_seconds, 7);
    assert_eq!(custom.reset_after_successes, 11);
    assert_eq!(custom.failure_statuses, vec![418u16]);
}
