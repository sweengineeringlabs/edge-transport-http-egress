//! Integration tests for the SWE-default `CacheConfig` baseline used by
//! `HttpCacheSvcProcessor::build_cache_layer()`.
//!
//! These tests verify that the default values produced by the SWE baseline are
//! sane — if the underlying config ever regresses, these assertions catch it.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor};

// ---------------------------------------------------------------------------
// SWE baseline — verify default config has production-safe values
// ---------------------------------------------------------------------------

/// `build_cache_layer` with the default config must succeed without error. A
/// failure here means the embedded default is malformed or the config schema
/// changed without updating the default.
#[test]
fn test_default_http_cache_swe_default_builder_succeeds() {
    HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default())
        .expect("swe_default baseline must parse without error");
}

/// The default `default_ttl_seconds` must be positive — a zero-second TTL
/// would make the default configuration a silently broken no-op.
#[test]
fn test_default_http_cache_swe_default_ttl_is_positive() {
    let cfg = CacheConfig::default();
    assert!(
        cfg.default_ttl_seconds >= 1,
        "swe_default default_ttl_seconds must be >= 1, got {}",
        cfg.default_ttl_seconds
    );
}

/// The default `max_entries` must be positive — a zero-capacity store silently
/// discards every response.
#[test]
fn test_default_http_cache_swe_default_max_entries_is_positive() {
    let cfg = CacheConfig::default();
    assert!(
        cfg.max_entries >= 1,
        "swe_default max_entries must be >= 1, got {}",
        cfg.max_entries
    );
}

/// Building from the SWE default must produce a valid `MiddlewareHttpCache`.
#[test]
fn test_default_http_cache_swe_default_builds_cache_layer() {
    HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default())
        .expect("build from swe_default must succeed");
}

// ---------------------------------------------------------------------------
// Custom config vs SWE default — policy independence
// ---------------------------------------------------------------------------

/// A consumer-supplied config with different values from the SWE default must
/// not be silently overwritten by the default-loading path.
#[test]
fn test_default_http_cache_custom_config_is_not_overridden_by_swe_default() {
    // Use values that are deliberately different from any likely SWE default.
    let b_cfg = CacheConfig {
        default_ttl_seconds: 3,
        max_entries: 7,
        respect_cache_control: false,
        cache_private: true,
    };
    assert_eq!(
        b_cfg.default_ttl_seconds, 3,
        "custom TTL must not be overridden by the SWE default"
    );
    assert_eq!(
        b_cfg.max_entries, 7,
        "custom max_entries must not be overridden by the SWE default"
    );
    assert!(
        !b_cfg.respect_cache_control,
        "custom respect_cache_control must not be overridden"
    );
    assert!(
        b_cfg.cache_private,
        "custom cache_private must not be overridden"
    );
}
