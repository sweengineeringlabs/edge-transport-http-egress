//! Integration tests for `StdCacheFactory`, the standard `CacheFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::ConfigBuilder as _;
use edge_transport_http_egress_cache_api::{
    CacheConfig, CacheError, CacheFactory, ConfigValidationRequest, DescribeRequest,
    FallbackTtlRequest,
};
use edge_transport_http_egress_cache_std::StdCacheFactory;

fn config_with_ttl(default_ttl_seconds: u64) -> CacheConfig {
    CacheConfig {
        default_ttl_seconds,
        ..CacheConfig::default()
    }
}

/// @covers: create_processor
#[test]
fn test_create_processor_describes_the_http_cache_happy() {
    let resp = StdCacheFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(resp.value, "http-cache");
}

/// @covers: create_validator
#[test]
fn test_create_validator_accepts_the_default_config_happy() {
    let result = StdCacheFactory
        .create_validator()
        .validate(ConfigValidationRequest {
            config: CacheConfig::default(),
        });
    assert!(
        matches!(result, Ok(())),
        "the default config must validate, got: {result:?}"
    );
}

/// @covers: create_validator
#[test]
fn test_create_validator_rejects_zero_max_entries_error() {
    let err = StdCacheFactory
        .create_validator()
        .validate(ConfigValidationRequest {
            config: CacheConfig {
                max_entries: 0,
                ..CacheConfig::default()
            },
        })
        .expect_err("a cache that holds no entries is invalid");
    assert!(
        matches!(&err, CacheError::InvalidConfig(msg) if msg.contains("max_entries")),
        "the error must name the field, got: {err:?}"
    );
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_seeds_the_package_name_happy() {
    let builder = StdCacheFactory.create_config_builder();
    // The name is a config search directory segment, so it is the package name.
    assert_eq!(builder.name(), "edge-transport-http-egress-cache-std");
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
}

/// @covers: build_cache_layer
#[test]
fn test_build_cache_layer_reports_the_configured_fallback_ttl_happy() {
    let layer = StdCacheFactory
        .build_cache_layer(config_with_ttl(77))
        .expect("a valid config must build");
    let resp = layer
        .default_ttl(FallbackTtlRequest)
        .expect("default_ttl is infallible");
    assert_eq!(resp.seconds, 77, "the layer must carry the configured TTL");
}

/// @covers: build_cache_layer
#[test]
fn test_build_cache_layer_called_twice_builds_independent_layers_edge() {
    let a = StdCacheFactory
        .build_cache_layer(config_with_ttl(5))
        .unwrap();
    let b = StdCacheFactory
        .build_cache_layer(config_with_ttl(500))
        .unwrap();
    assert_eq!(a.default_ttl(FallbackTtlRequest).unwrap().seconds, 5);
    assert_eq!(b.default_ttl(FallbackTtlRequest).unwrap().seconds, 500);
}
