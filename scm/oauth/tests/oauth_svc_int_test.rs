//! Integration tests for `OAuthSvc` — extracted from saf/oauth_svc.rs.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_oauth::{OAuthBuilderOps as _, OAuthSvc};

/// @covers: OAuthSvc::builder
#[test]
fn test_builder_without_source_returns_configuration_error() {
    let result = OAuthSvc::builder().build();
    assert!(result.is_err(), "build without token source must fail");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("no OAuthTokenSource"),
        "error must identify missing source: {msg}",
    );
}

/// @covers: OAuthSvc::create_config_builder
#[test]
fn test_create_config_builder_builds_loader() {
    let _loader = OAuthSvc::create_config_builder().build_loader();
}
