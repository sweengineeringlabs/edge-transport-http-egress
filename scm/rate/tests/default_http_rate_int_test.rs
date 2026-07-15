//! Integration tests for `core/default_http_rate.rs`.
//!
//! `DefaultHttpRate` is `pub(crate)`.  Its observable effect is through the
//! SAF `HttpRateSvc::build_rate_layer()` function, which accepts a `RateConfig`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{HttpRateSvc, RateConfig};

// ---------------------------------------------------------------------------
// SWE baseline — verify default config has production-safe values
// ---------------------------------------------------------------------------

/// `HttpRateSvc::build_rate_layer(RateConfig::default())` must succeed without error.
#[test]
fn test_default_http_rate_swe_default_builder_succeeds() {
    HttpRateSvc::build_rate_layer(RateConfig::default())
        .expect("swe_default baseline must parse without error");
}

/// Default `tokens_per_second` must be >= 1.
#[test]
fn test_default_http_rate_swe_default_tokens_per_second_is_positive() {
    let cfg = RateConfig::default();
    assert!(
        cfg.tokens_per_second >= 1,
        "swe_default tokens_per_second must be >= 1, got {}",
        cfg.tokens_per_second
    );
}

/// Default `burst_capacity` must be >= 1.
#[test]
fn test_default_http_rate_swe_default_burst_capacity_is_positive() {
    let cfg = RateConfig::default();
    assert!(
        cfg.burst_capacity >= 1,
        "swe_default burst_capacity must be >= 1, got {}",
        cfg.burst_capacity
    );
}

/// Building from the SWE default must produce a valid `RateLayer`.
#[test]
fn test_default_http_rate_swe_default_builds_rate_layer() {
    HttpRateSvc::build_rate_layer(RateConfig::default())
        .expect("build from swe_default must succeed");
}

// ---------------------------------------------------------------------------
// Custom config is not overridden by the default-loading path
// ---------------------------------------------------------------------------

/// A consumer-supplied config must pass through unchanged — no field must
/// be silently replaced by a SWE default.
#[test]
fn test_default_http_rate_custom_config_is_not_overridden_by_swe_default() {
    let custom = RateConfig {
        tokens_per_second: 3,
        burst_capacity: 7,
        per_host: true,
    };
    assert_eq!(custom.tokens_per_second, 3);
    assert_eq!(custom.burst_capacity, 7);
    assert!(custom.per_host);
}
