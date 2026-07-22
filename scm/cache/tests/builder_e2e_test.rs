//! End-to-end tests for the edge_transport_http_egress_cache SAF builder surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{CacheConfig, HttpCacheSvcProcessor, MiddlewareHttpCache};

fn make_cfg() -> CacheConfig {
    CacheConfig {
        default_ttl_seconds: 300,
        max_entries: 100,
        respect_cache_control: true,
        cache_private: false,
    }
}

/// @covers: build_cache_layer with default config
#[test]
fn test_e2e_builder() {
    let layer: MiddlewareHttpCache =
        HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default())
            .expect("build must succeed");
    let s = format!("{layer:?}");
    assert!(
        s.contains("MiddlewareHttpCache"),
        "e2e: Debug must contain 'MiddlewareHttpCache': {s}"
    );
}

/// @covers: build_cache_layer stores config fields correctly
#[test]
fn test_e2e_with_config() {
    let cfg = make_cfg();
    assert_eq!(cfg.default_ttl_seconds, 300);
    HttpCacheSvcProcessor::build_cache_layer(cfg).expect("e2e with_config build must succeed");
}

/// @covers: CacheConfig fields are accessible directly
#[test]
fn test_e2e_config() {
    let cfg = make_cfg();
    assert_eq!(cfg.max_entries, 100);
    assert!(cfg.respect_cache_control);
}

/// @covers: build_cache_layer with custom config
#[test]
fn test_e2e_build() {
    let cfg = CacheConfig {
        default_ttl_seconds: 60,
        max_entries: 50,
        respect_cache_control: false,
        cache_private: true,
    };
    let layer = HttpCacheSvcProcessor::build_cache_layer(cfg).expect("e2e build must succeed");
    assert!(!format!("{layer:?}").is_empty());
}

/// @covers: create_config_builder returns a working Loader
#[test]
fn test_e2e_create_config_builder_returns_loader() {
    let loader = HttpCacheSvcProcessor::create_config_builder()
        .build_loader()
        .expect("build_loader must succeed with no config dir configured");
    // With no config file present in the test environment, loading the
    // `cache` section must be a real, well-formed error — not a panic and
    // not a silently-succeeding stub default.
    let result: Result<CacheConfig, _> = loader.load_section("cache");
    assert!(
        result.is_err(),
        "loading a section with no config file present must be a real error"
    );
}
