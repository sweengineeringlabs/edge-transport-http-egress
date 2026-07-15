//! End-to-end tests for the edge_transport_http_egress_auth SAF builder surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::{AuthConfig, AuthMiddleware, AuthSvc};

/// @covers: build_auth_middleware with None config
#[test]
fn test_e2e_build_none_config() {
    let mw: AuthMiddleware =
        AuthSvc::build_auth_middleware(AuthConfig::None).expect("None config must always build");
    let s = format!("{mw:?}");
    assert!(
        s.contains("http-auth"),
        "e2e: middleware Debug must name crate: {s}"
    );
}

/// @covers: build_auth_middleware with None — config is the value passed in
#[test]
fn test_e2e_none_config_value() {
    let cfg = AuthConfig::None;
    assert!(matches!(cfg, AuthConfig::None));
    let mw = AuthSvc::build_auth_middleware(cfg).expect("None config must build");
    assert!(!format!("{mw:?}").is_empty());
}

/// @covers: build_auth_middleware with Bearer when env is set
#[test]
fn test_e2e_build_bearer_with_env_set() {
    let env = "SWE_E2E_AUTH_BEARER_01";
    std::env::set_var(env, "e2e-token");
    let cfg = AuthConfig::Bearer {
        token_env: env.into(),
    };
    let mw =
        AuthSvc::build_auth_middleware(cfg).expect("bearer e2e build must succeed when env set");
    assert!(!format!("{mw:?}").is_empty());
    std::env::remove_var(env);
}

/// @covers: create_config_builder returns a Loader
#[test]
fn test_e2e_create_config_builder_returns_loader() {
    let _loader = AuthSvc::create_config_builder().build_loader();
}
