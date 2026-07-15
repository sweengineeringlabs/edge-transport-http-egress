//! Integration tests for `AuthSvc` public API — extracted from saf/auth_svc.rs.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::{AuthConfig, AuthError, AuthSvc};

/// @covers: AuthSvc::create_config_builder
#[test]
fn test_create_config_builder_builds_loader() {
    let _loader = AuthSvc::create_config_builder().build_loader();
}

/// @covers: AuthSvc::build_auth_middleware
#[test]
fn test_build_auth_middleware_with_none_config_returns_middleware_instance() {
    let mw = AuthSvc::build_auth_middleware(AuthConfig::None).expect("build ok");
    let s = format!("{mw:?}");
    assert!(s.contains("http-auth"));
}

/// @covers: AuthSvc::build_auth_middleware
#[test]
fn test_build_auth_middleware_with_missing_bearer_env_fails_at_build_time() {
    let cfg = AuthConfig::Bearer {
        token_env: "EDGE_TEST_DEFINITELY_NOT_SET_99".into(),
    };
    std::env::remove_var("EDGE_TEST_DEFINITELY_NOT_SET_99");
    let err = AuthSvc::build_auth_middleware(cfg).unwrap_err();
    match err {
        AuthError::MissingEnvVar { name } => {
            assert_eq!(name, "EDGE_TEST_DEFINITELY_NOT_SET_99");
        }
        other => panic!("expected MissingEnvVar, got {other:?}"),
    }
}
