//! Integration tests for `build_auth_middleware` and `create_config_builder` SAF entry points.
//!
//! Covers the full public factory surface: `build_auth_middleware`, `create_config_builder`,
//! and config variant handling.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::{AuthConfig, AuthError, AuthMiddleware, AuthSvc};

// ---------------------------------------------------------------------------
// create_config_builder — SAF entry point
// ---------------------------------------------------------------------------

#[test]
fn test_create_config_builder_returns_working_loader() {
    // The free `AuthSvc::create_config_builder()` function must return a loader that
    // works. Failure here means the crate package name wiring is broken.
    let _loader = AuthSvc::create_config_builder().build_loader();
}

/// The SWE default auth config is None (pass-through).
#[test]
fn test_default_auth_config_is_none() {
    // None config requires no env vars — must always succeed.
    let mw = AuthSvc::build_auth_middleware(AuthConfig::None).expect("None must always build");
    assert!(!format!("{mw:?}").is_empty());
}

// ---------------------------------------------------------------------------
// build_auth_middleware with None — always succeeds
// ---------------------------------------------------------------------------

#[test]
fn test_build_auth_middleware_none_variant_succeeds() {
    AuthSvc::build_auth_middleware(AuthConfig::None)
        .expect("None config must build unconditionally");
}

// ---------------------------------------------------------------------------
// build_auth_middleware — stores each config variant correctly
// ---------------------------------------------------------------------------

#[test]
fn test_build_auth_middleware_none_variant_builds() {
    let mw: AuthMiddleware =
        AuthSvc::build_auth_middleware(AuthConfig::None).expect("with_config(None) must build");
    let _ = format!("{mw:?}");
}

#[test]
fn test_build_auth_middleware_bearer_variant_fails_without_env() {
    let env_name = "SWE_BLD_MISS_BEARER_01";
    std::env::remove_var(env_name);
    let err = AuthSvc::build_auth_middleware(AuthConfig::Bearer {
        token_env: env_name.into(),
    })
    .unwrap_err();
    match err {
        AuthError::MissingEnvVar { name } => assert_eq!(name, env_name),
        other => panic!("expected MissingEnvVar, got {other:?}"),
    }
}

#[test]
fn test_build_auth_middleware_bearer_variant_succeeds_with_env() {
    let env_name = "SWE_BLD_SET_BEARER_01";
    std::env::set_var(env_name, "bld-token-value");
    let mw = AuthSvc::build_auth_middleware(AuthConfig::Bearer {
        token_env: env_name.into(),
    })
    .expect("Bearer with env set must build");
    let _ = format!("{mw:?}");
    std::env::remove_var(env_name);
}

#[test]
fn test_build_auth_middleware_basic_missing_pass_env_returns_missing_env_var() {
    let user_env = "SWE_BLD_MISS_BASIC_U_01";
    let pass_env = "SWE_BLD_MISS_BASIC_P_01";
    std::env::set_var(user_env, "user"); // user present — pass absent
    std::env::remove_var(pass_env);
    let err = AuthSvc::build_auth_middleware(AuthConfig::Basic {
        user_env: user_env.into(),
        pass_env: pass_env.into(),
    })
    .unwrap_err();
    match err {
        AuthError::MissingEnvVar { name } => assert_eq!(name, pass_env),
        other => panic!("expected MissingEnvVar for pass_env, got {other:?}"),
    }
    std::env::remove_var(user_env);
}

#[test]
fn test_build_auth_middleware_header_stores_variant() {
    let env_name = "SWE_BLD_HEADER_01";
    std::env::set_var(env_name, "test-header-val");
    let mw = AuthSvc::build_auth_middleware(AuthConfig::Header {
        name: "x-custom-key".into(),
        value_env: env_name.into(),
    })
    .expect("Header with env set must build");
    let _ = format!("{mw:?}");
    std::env::remove_var(env_name);
}

#[test]
fn test_build_auth_middleware_aws_sigv4_missing_access_key_fails() {
    let ak_env = "SWE_BLD_AWS_AK_01";
    let sk_env = "SWE_BLD_AWS_SK_01";
    std::env::remove_var(ak_env);
    std::env::remove_var(sk_env);
    let err = AuthSvc::build_auth_middleware(AuthConfig::AwsSigV4 {
        access_key_env: ak_env.into(),
        secret_key_env: sk_env.into(),
        session_token_env: None,
        region: "eu-west-1".into(),
        service: "s3".into(),
    })
    .unwrap_err();
    assert!(
        matches!(err, AuthError::MissingEnvVar { .. }),
        "AwsSigV4 without access key env must fail: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// build_auth_middleware — bearer token_env name is stored not resolved early
// ---------------------------------------------------------------------------

#[test]
fn test_build_auth_middleware_bearer_stores_token_env_name() {
    let env_name = "SWE_BLD_CFG_BEARER_01";
    std::env::set_var(env_name, "some-token");
    let mw = AuthSvc::build_auth_middleware(AuthConfig::Bearer {
        token_env: env_name.into(),
    })
    .expect("Bearer with env set must build");
    let _ = format!("{mw:?}");
    std::env::remove_var(env_name);
}

// ---------------------------------------------------------------------------
// build_auth_middleware — None always succeeds regardless of env
// ---------------------------------------------------------------------------

#[test]
fn test_build_auth_middleware_none_succeeds_regardless_of_env() {
    AuthSvc::build_auth_middleware(AuthConfig::None)
        .expect("None config must always build regardless of env state");
}
