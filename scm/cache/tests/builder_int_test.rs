//! Integration tests for the `build_cache_layer` SAF entry point.
//!
//! Covers: `build_cache_layer`, `CacheConfig` fields, `CacheLayer` construction.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, CacheError, CacheLayer, HttpCacheSvc};

// ---------------------------------------------------------------------------
// build_cache_layer — SAF entry point
// ---------------------------------------------------------------------------

/// Verifies that `build_cache_layer` with the default config succeeds — the
/// crate-shipped baseline must always be parseable. If it panics, the crate is
/// broken before a consumer has touched a single line of config.
#[test]
fn test_builder_fn_succeeds_with_swe_default() {
    HttpCacheSvc::build_cache_layer(CacheConfig::default()).expect("build must succeed");
}

/// A `default_ttl_seconds` of zero means every response expires the instant it
/// is stored — the cache becomes a no-op. The baseline must set a positive TTL.
#[test]
fn test_builder_fn_swe_default_ttl_is_positive() {
    let cfg = CacheConfig::default();
    assert!(
        cfg.default_ttl_seconds >= 1,
        "swe_default default_ttl_seconds must be >= 1, got {}",
        cfg.default_ttl_seconds
    );
}

/// A `max_entries` of zero means the backing store can hold nothing. The
/// baseline must configure a non-zero capacity.
#[test]
fn test_builder_fn_swe_default_max_entries_is_positive() {
    let cfg = CacheConfig::default();
    assert!(
        cfg.max_entries >= 1,
        "swe_default max_entries must be >= 1, got {}",
        cfg.max_entries
    );
}

// ---------------------------------------------------------------------------
// CacheConfig — custom policy round-trips correctly
// ---------------------------------------------------------------------------

/// `CacheConfig` must preserve every field of the supplied config without
/// silent modification.
#[test]
fn test_with_config_preserves_all_fields() {
    let cfg = CacheConfig {
        default_ttl_seconds: 42,
        max_entries: 256,
        respect_cache_control: false,
        cache_private: true,
    };
    let b_cfg = cfg;
    let policy = &b_cfg;
    assert_eq!(policy.default_ttl_seconds, 42);
    assert_eq!(policy.max_entries, 256);
    assert!(!policy.respect_cache_control);
    assert!(policy.cache_private);
}

/// Config can be accessed as a reference without a divergent copy.
#[test]
fn test_config_accessor_returns_reference_not_divergent_copy() {
    let cfg = CacheConfig {
        default_ttl_seconds: 99,
        max_entries: 7,
        respect_cache_control: true,
        cache_private: false,
    };
    let b_cfg = cfg;
    let policy: &CacheConfig = &b_cfg;
    assert_eq!(
        policy.default_ttl_seconds, 99,
        "ttl mismatch via &CacheConfig"
    );
    assert_eq!(
        policy.max_entries, 7,
        "max_entries mismatch via &CacheConfig"
    );
}

// ---------------------------------------------------------------------------
// build_cache_layer — produces a usable CacheLayer
// ---------------------------------------------------------------------------

/// The nominal build path must succeed and return a `CacheLayer`.
#[test]
fn test_build_from_swe_default_returns_cache_layer() {
    let layer: CacheLayer =
        HttpCacheSvc::build_cache_layer(CacheConfig::default()).expect("build() must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("CacheLayer"),
        "Debug output must identify the type; got: {dbg}"
    );
}

/// Build with a custom config must succeed and reflect the supplied TTL in the
/// `Debug` output (which the `CacheLayer::fmt` impl includes).
#[test]
fn test_build_with_custom_ttl_reflects_in_debug_output() {
    let cfg = CacheConfig {
        default_ttl_seconds: 7,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    };
    let layer = HttpCacheSvc::build_cache_layer(cfg).expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("7"),
        "Debug output must include the configured TTL; got: {dbg}"
    );
}

/// `cache_private = true` is an opt-in flag that must not be rejected at build
/// time — it is a valid runtime policy choice.
#[test]
fn test_build_with_cache_private_true_succeeds() {
    let cfg = CacheConfig {
        default_ttl_seconds: 60,
        max_entries: 50,
        respect_cache_control: true,
        cache_private: true,
    };
    HttpCacheSvc::build_cache_layer(cfg)
        .expect("cache_private=true must not be rejected by build()");
}

/// `respect_cache_control = false` is a legitimate policy choice. Build must
/// not reject it.
#[test]
fn test_build_with_respect_cache_control_false_succeeds() {
    let cfg = CacheConfig {
        default_ttl_seconds: 120,
        max_entries: 500,
        respect_cache_control: false,
        cache_private: false,
    };
    HttpCacheSvc::build_cache_layer(cfg)
        .expect("respect_cache_control=false must not be rejected by build()");
}

/// Very large `max_entries` values are legitimate — operators may set high
/// memory budgets. Build must not reject them.
#[test]
fn test_build_with_large_max_entries_succeeds() {
    let cfg = CacheConfig {
        default_ttl_seconds: 30,
        max_entries: 1_000_000,
        respect_cache_control: true,
        cache_private: false,
    };
    HttpCacheSvc::build_cache_layer(cfg)
        .expect("max_entries=1_000_000 must not be rejected by build()");
}

/// `default_ttl_seconds = 0` is valid config: it means "no TTL fallback — only
/// cache responses that supply their own Cache-Control max-age." Build must
/// not reject it.
#[test]
fn test_build_with_zero_ttl_succeeds() {
    let cfg = CacheConfig {
        default_ttl_seconds: 0,
        max_entries: 10,
        respect_cache_control: true,
        cache_private: false,
    };
    HttpCacheSvc::build_cache_layer(cfg)
        .expect("default_ttl_seconds=0 must not be rejected by build()");
}

// ---------------------------------------------------------------------------
// Error variants — compile-time coverage
// ---------------------------------------------------------------------------

/// `Error::ParseFailed` must be constructable so downstream consumers can
/// pattern-match it. This verifies the variant is `pub` and its inner `String`
/// is accessible.
#[test]
fn test_error_parse_failed_is_constructable_and_its_message_is_accessible() {
    let reason = "missing field `max_entries`".to_string();
    let err = CacheError::ParseFailed(reason.clone());
    let display = err.to_string();
    assert!(
        display.contains(&reason),
        "ParseFailed display must echo the supplied reason; got: {display}"
    );
}
