//! Integration tests for `OAuthSvc` type.
//!
//! Rule 120: `src/api/types/o_auth_svc.rs` requires a corresponding test file.

use edge_transport_http_egress_oauth::{OAuthBuilderOps, OAuthSvc};

/// @covers: OAuthSvc::builder
/// `OAuthSvc::builder()` must return an `OAuthBuilder` (verified by calling build()).
#[test]
fn oauth_struct_o_auth_svc_builder_returns_builder_int_test() {
    let result = OAuthSvc::builder().build();
    // Without a token source the build must fail — proving builder() returned
    // a real OAuthBuilder rather than some other type.
    assert!(
        result.is_err(),
        "OAuthSvc::builder().build() without source must fail"
    );
}

/// @covers: OAuthSvc::create_config_builder
/// `create_config_builder()` must return a builder that can produce a loader.
#[test]
fn oauth_struct_o_auth_svc_create_config_builder_builds_loader_int_test() {
    let _loader = OAuthSvc::create_config_builder().build_loader();
}
