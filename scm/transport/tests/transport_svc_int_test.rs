//! Integration tests for transport SAF factory functions not covered elsewhere.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpTransportSvc;

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_builder_with_package_name() {
    let builder = HttpTransportSvc::create_config_builder();
    let name = builder.name();
    assert!(
        !name.is_empty(),
        "config builder must carry the package name"
    );
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_builder_with_package_version() {
    let builder = HttpTransportSvc::create_config_builder();
    let version = builder.version();
    assert!(
        !version.is_empty(),
        "config builder must carry the package version"
    );
}
