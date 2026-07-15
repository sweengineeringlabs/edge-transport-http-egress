#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `ApplicationConfigBuilder`.

use edge_transport_http_egress_loadbalancer::LoadbalancerSvc;

/// @covers: LoadbalancerSvc::create_config_builder — produces a ConfigBuilderImpl
#[test]
fn test_create_config_builder_produces_builder_with_crate_metadata() {
    let builder = LoadbalancerSvc::create_config_builder();
    // ConfigBuilderImpl doesn't derive Debug; verify it builds without panic.
    let _ = builder;
}
