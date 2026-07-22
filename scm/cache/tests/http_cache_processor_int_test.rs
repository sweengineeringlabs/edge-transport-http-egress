//! Integration tests for `core/processor/http_cache_processor.rs` — the
//! `Processor` impl and factory surface on `HttpCacheSvcProcessor`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{
    CacheConfig, DescribeRequest, HttpCacheSvcProcessor, Processor,
};

/// @covers: describe
#[test]
fn test_describe_returns_stable_label_across_instances_happy() {
    let a = HttpCacheSvcProcessor
        .describe(DescribeRequest)
        .expect("infallible");
    let b = HttpCacheSvcProcessor
        .describe(DescribeRequest)
        .expect("infallible");
    assert_eq!(a.value, "http-cache");
    assert_eq!(a.value, b.value);
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_seeds_crate_version_happy() {
    let builder = HttpCacheSvcProcessor::create_config_builder();
    assert_eq!(builder.name(), "edge-transport-http-egress-cache");
}

/// @covers: build_cache_layer
#[test]
fn test_build_cache_layer_rejects_nothing_but_reflects_input_edge() {
    let cfg = CacheConfig {
        default_ttl_seconds: 7,
        max_entries: 3,
        respect_cache_control: false,
        cache_private: true,
    };
    let layer = HttpCacheSvcProcessor::build_cache_layer(cfg).expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(dbg.contains("7"), "must reflect default_ttl_seconds=7");
    assert!(dbg.contains("max_entries: 3"), "must reflect max_entries=3");
    assert!(
        dbg.contains("cache_private: true"),
        "must reflect cache_private=true"
    );
}
