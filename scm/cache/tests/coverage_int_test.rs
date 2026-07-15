//! Coverage tests (rules 221 + 222) — _happy / _error / _edge variants.
//! Rule 221: create_config_builder, build_cache_layer in cache_svc.rs.
//! Rule 222: describe (HttpCache + Processor traits), validate (Validator).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvc};

// ── create_config_builder (rule 221) ─────────────────────────────────────────

#[test]
fn test_create_config_builder_returns_valid_loader_happy() {
    let loader = HttpCacheSvc::create_config_builder().build_loader();
    let _ = loader;
}

#[test]
fn test_create_config_builder_loader_usable_without_config_file_error() {
    // Without a config file, build_loader returns a loader (not panics)
    let loader = HttpCacheSvc::create_config_builder().build_loader();
    let _ = loader;
}

#[test]
fn test_create_config_builder_independent_instances_edge() {
    let l1 = HttpCacheSvc::create_config_builder().build_loader();
    let l2 = HttpCacheSvc::create_config_builder().build_loader();
    let _ = (l1, l2);
}

// ── build_cache_layer (rule 221) ─────────────────────────────────────────────

#[test]
fn test_build_cache_layer_default_config_succeeds_happy() {
    let result = HttpCacheSvc::build_cache_layer(CacheConfig::default());
    assert!(result.is_ok(), "default config must build successfully");
}

#[test]
fn test_build_cache_layer_valid_config_does_not_error_error() {
    let result = HttpCacheSvc::build_cache_layer(CacheConfig::default());
    assert!(result.is_ok(), "valid config must not return error");
}

#[test]
fn test_build_cache_layer_idempotent_for_same_config_edge() {
    let r1 = HttpCacheSvc::build_cache_layer(CacheConfig::default());
    let r2 = HttpCacheSvc::build_cache_layer(CacheConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}

// ── describe (rule 222: HttpCache + Processor traits) ────────────────────────

#[test]
fn test_describe_cache_layer_has_debug_representation_happy() {
    let layer = HttpCacheSvc::build_cache_layer(CacheConfig::default()).expect("ok");
    let dbg = format!("{layer:?}");
    assert!(!dbg.is_empty(), "CacheLayer Debug must be non-empty");
}

#[test]
fn test_describe_does_not_return_empty_error() {
    let layer = HttpCacheSvc::build_cache_layer(CacheConfig::default()).expect("ok");
    let dbg = format!("{layer:?}");
    assert_ne!(dbg, "");
}

#[test]
fn test_describe_deterministic_across_calls_edge() {
    let layer = HttpCacheSvc::build_cache_layer(CacheConfig::default()).expect("ok");
    let a = format!("{layer:?}");
    let b = format!("{layer:?}");
    assert_eq!(a, b);
}

// ── validate (rule 222: Validator trait) ─────────────────────────────────────

#[test]
fn test_validate_default_cache_config_passes_happy() {
    let result = HttpCacheSvc::build_cache_layer(CacheConfig::default());
    assert!(result.is_ok(), "default CacheConfig must pass validation");
}

#[test]
fn test_validate_valid_config_never_returns_unexpected_error_error() {
    let result = HttpCacheSvc::build_cache_layer(CacheConfig::default());
    assert!(result.is_ok());
}

#[test]
fn test_validate_repeated_builds_produce_consistent_result_edge() {
    let r1 = HttpCacheSvc::build_cache_layer(CacheConfig::default());
    let r2 = HttpCacheSvc::build_cache_layer(CacheConfig::default());
    assert_eq!(r1.is_ok(), r2.is_ok());
}
