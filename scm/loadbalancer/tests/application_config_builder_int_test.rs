#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `ApplicationConfigBuilder`.
//!
//! `ApplicationConfigBuilder` is a crate-internal alias for the config type
//! resolved from `config/application.toml`. These tests verify it is genuinely
//! wired to that file via `LoadbalancerSvcProcessor::create_config_builder()`.

use edge_transport_http_egress_loadbalancer::LoadbalancerSvcProcessor;

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_carries_this_crate_name() {
    let builder = LoadbalancerSvcProcessor::create_config_builder();
    assert_eq!(
        builder.name(),
        "edge-transport-http-egress-loadbalancer",
        "the builder seeded for config/application.toml must carry this crate's own name"
    );
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_version_is_non_empty() {
    let builder = LoadbalancerSvcProcessor::create_config_builder();
    assert!(!builder.version().is_empty());
}
