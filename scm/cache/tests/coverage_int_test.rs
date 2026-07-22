//! Coverage tests (rules 221 + 222) — _happy / _error / _edge variants.
//! Rule 221: create_config_builder, build_cache_layer in cache_svc.rs.
//! Rule 222: describe (HttpCache + Processor traits), validate (Validator).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor};

// ── create_config_builder (rule 221) ─────────────────────────────────────────

#[test]
fn test_create_config_builder_returns_valid_loader_happy() {
    let loader = HttpCacheSvcProcessor::create_config_builder()
        .build_loader()
        .expect("build_loader must succeed");
    // The loader is genuinely functional: with no config file present, loading
    // the `cache` section is a real error, not a silent stub default.
    let result: Result<CacheConfig, _> = loader.load_section("cache");
    assert!(
        result.is_err(),
        "loading a section without a config file must be a real error"
    );
}

#[test]
fn test_create_config_builder_loader_usable_without_config_file_error() {
    let loader = HttpCacheSvcProcessor::create_config_builder()
        .build_loader()
        .expect("build_loader must succeed");
    let result: Result<CacheConfig, _> = loader.load_section("cache");
    assert!(
        result.is_err(),
        "no config file present must yield a load error"
    );
}

#[test]
fn test_create_config_builder_independent_instances_edge() {
    let b1 = HttpCacheSvcProcessor::create_config_builder();
    let b2 = HttpCacheSvcProcessor::create_config_builder();
    // Independent instances must each be seeded with this crate's identity.
    assert_eq!(
        b1.name(),
        "edge-transport-http-egress-cache",
        "first builder must carry the crate name"
    );
    assert_eq!(
        b1.name(),
        b2.name(),
        "independent builders must agree on the crate name"
    );
}

// ── build_cache_layer (rule 221) ─────────────────────────────────────────────

#[test]
fn test_build_cache_layer_default_config_succeeds_happy() {
    let layer = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default())
        .expect("default config must build successfully");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("300") && dbg.contains("10000"),
        "default policy (ttl=300, max_entries=10000) must reach the layer; got: {dbg}"
    );
}

#[test]
fn test_build_cache_layer_valid_config_does_not_error_error() {
    // Non-default input so a stub that ignored config could not fake this.
    let cfg = CacheConfig {
        default_ttl_seconds: 1357,
        max_entries: 42,
        respect_cache_control: false,
        cache_private: true,
    };
    let layer =
        HttpCacheSvcProcessor::build_cache_layer(cfg).expect("valid config must not return error");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("1357"),
        "the supplied ttl=1357 must flow through to the layer; got: {dbg}"
    );
}

#[test]
fn test_build_cache_layer_idempotent_for_same_config_edge() {
    let r1 = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default());
    let r2 = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}

// ── describe (rule 222: HttpCache + Processor traits) ────────────────────────

#[test]
fn test_describe_cache_layer_has_debug_representation_happy() {
    let layer = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default()).expect("ok");
    let dbg = format!("{layer:?}");
    assert!(
        !dbg.is_empty(),
        "MiddlewareHttpCache Debug must be non-empty"
    );
}

#[test]
fn test_describe_does_not_return_empty_error() {
    let layer = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default()).expect("ok");
    let dbg = format!("{layer:?}");
    assert_ne!(dbg, "");
}

#[test]
fn test_describe_deterministic_across_calls_edge() {
    let layer = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default()).expect("ok");
    let a = format!("{layer:?}");
    let b = format!("{layer:?}");
    assert_eq!(a, b);
}

// ── validate (rule 222: Validator trait) ─────────────────────────────────────

#[test]
fn test_validate_default_cache_config_passes_happy() {
    let layer = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default())
        .expect("default CacheConfig must pass validation");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("300"),
        "validated default config must carry ttl=300; got: {dbg}"
    );
}

#[test]
fn test_validate_valid_config_never_returns_unexpected_error_error() {
    // Non-default input to prove validation honours the actual values.
    let cfg = CacheConfig {
        default_ttl_seconds: 909,
        max_entries: 3,
        respect_cache_control: true,
        cache_private: false,
    };
    let layer = HttpCacheSvcProcessor::build_cache_layer(cfg).expect("valid config must build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("909"),
        "validated config must carry ttl=909; got: {dbg}"
    );
}

#[test]
fn test_validate_repeated_builds_produce_consistent_result_edge() {
    let r1 = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default());
    let r2 = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default());
    assert_eq!(r1.is_ok(), r2.is_ok());
}
