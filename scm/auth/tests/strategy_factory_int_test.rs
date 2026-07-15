//! Integration tests for `strategy_factory::build_strategy` effects.
//!
//! `build_strategy` is `pub(crate)`.  Its effect is the selection of the
//! correct concrete strategy for each `AuthConfig` variant, observable through:
//! 1. `AuthSvc::build_auth_middleware()` success/failure (each variant has different env deps).
//! 2. The header attached to a request (indirect — we can't call `authorize()`
//!    from outside, but we can observe `AuthMiddleware::handle()` effects by
//!    wiring the middleware).
//!
//! For header-value correctness, the per-strategy test files apply.  This
//! file verifies the factory's dispatch: "config variant X → correct strategy
//! selected → correct build outcome."
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::{AuthConfig, AuthError, AuthSvc};

// ---------------------------------------------------------------------------
// None → NoopStrategy (no env needed, no header attached)
// ---------------------------------------------------------------------------

#[test]
fn test_factory_none_config_builds_without_env_vars() {
    AuthSvc::build_auth_middleware(AuthConfig::None)
        .expect("None→NoopStrategy must build without any env vars");
}

// ---------------------------------------------------------------------------
// Bearer → BearerStrategy (one env var)
// ---------------------------------------------------------------------------

#[test]
fn test_factory_bearer_config_fails_without_token_env() {
    let env_name = "SWE_AUTH_FACTORY_BRR_MISS_01";
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
fn test_factory_bearer_config_builds_with_token_env() {
    let env_name = "SWE_AUTH_FACTORY_BRR_OK_01";
    std::env::set_var(env_name, "factory-bearer-token");
    AuthSvc::build_auth_middleware(AuthConfig::Bearer {
        token_env: env_name.into(),
    })
    .expect("Bearer with env set must build");
    std::env::remove_var(env_name);
}

// ---------------------------------------------------------------------------
// Basic → BasicStrategy (two env vars)
// ---------------------------------------------------------------------------

#[test]
fn test_factory_basic_config_fails_without_user_env() {
    let user_env = "SWE_AUTH_FACTORY_BASIC_U_MISS_01";
    let pass_env = "SWE_AUTH_FACTORY_BASIC_P_MISS_01";
    std::env::remove_var(user_env);
    std::env::remove_var(pass_env);
    let err = AuthSvc::build_auth_middleware(AuthConfig::Basic {
        user_env: user_env.into(),
        pass_env: pass_env.into(),
    })
    .unwrap_err();
    assert!(
        matches!(err, AuthError::MissingEnvVar { .. }),
        "Basic without user env must fail: {err:?}"
    );
}

#[test]
fn test_factory_basic_config_builds_with_both_envs() {
    let user_env = "SWE_AUTH_FACTORY_BASIC_U_OK_01";
    let pass_env = "SWE_AUTH_FACTORY_BASIC_P_OK_01";
    std::env::set_var(user_env, "alice");
    std::env::set_var(pass_env, "wonderland");
    AuthSvc::build_auth_middleware(AuthConfig::Basic {
        user_env: user_env.into(),
        pass_env: pass_env.into(),
    })
    .expect("Basic with both envs must build");
    std::env::remove_var(user_env);
    std::env::remove_var(pass_env);
}

// ---------------------------------------------------------------------------
// Header → HeaderStrategy (one env var + header name validation)
// ---------------------------------------------------------------------------

#[test]
fn test_factory_header_config_fails_without_value_env() {
    let env_name = "SWE_AUTH_FACTORY_HDR_MISS_01";
    std::env::remove_var(env_name);
    let err = AuthSvc::build_auth_middleware(AuthConfig::Header {
        name: "x-api-key".into(),
        value_env: env_name.into(),
    })
    .unwrap_err();
    assert!(
        matches!(err, AuthError::MissingEnvVar { .. }),
        "Header without value env must fail: {err:?}"
    );
}

#[test]
fn test_factory_header_config_builds_with_valid_name_and_env() {
    let env_name = "SWE_AUTH_FACTORY_HDR_OK_01";
    std::env::set_var(env_name, "factory-api-key");
    AuthSvc::build_auth_middleware(AuthConfig::Header {
        name: "x-api-key".into(),
        value_env: env_name.into(),
    })
    .expect("Header with valid name + env must build");
    std::env::remove_var(env_name);
}

#[test]
fn test_factory_header_config_rejects_invalid_header_name_at_build() {
    let env_name = "SWE_AUTH_FACTORY_HDR_BADNAME_01";
    std::env::set_var(env_name, "some-value");
    let err = AuthSvc::build_auth_middleware(AuthConfig::Header {
        name: "bad name spaces".into(),
        value_env: env_name.into(),
    })
    .unwrap_err();
    assert!(
        matches!(err, AuthError::InvalidHeaderName { .. }),
        "Header with invalid name must produce InvalidHeaderName, got {err:?}"
    );
    std::env::remove_var(env_name);
}

// ---------------------------------------------------------------------------
// AwsSigV4 → AwsSigV4Strategy (two required + one optional env var)
// ---------------------------------------------------------------------------

#[test]
fn test_factory_aws_sigv4_config_fails_without_access_key_env() {
    let ak_env = "SWE_AUTH_FACTORY_AWS_AK_MISS_01";
    let sk_env = "SWE_AUTH_FACTORY_AWS_SK_MISS_01";
    std::env::remove_var(ak_env);
    std::env::remove_var(sk_env);
    let err = AuthSvc::build_auth_middleware(AuthConfig::AwsSigV4 {
        access_key_env: ak_env.into(),
        secret_key_env: sk_env.into(),
        session_token_env: None,
        region: "us-east-1".into(),
        service: "s3".into(),
    })
    .unwrap_err();
    assert!(
        matches!(err, AuthError::MissingEnvVar { .. }),
        "AwsSigV4 without access key env must fail: {err:?}"
    );
}

#[test]
fn test_factory_aws_sigv4_config_builds_with_required_envs() {
    let ak_env = "SWE_AUTH_FACTORY_AWS_AK_OK_01";
    let sk_env = "SWE_AUTH_FACTORY_AWS_SK_OK_01";
    std::env::set_var(ak_env, "AKIA_FACTORY_TEST");
    std::env::set_var(sk_env, "SECRET_FACTORY_TEST");
    AuthSvc::build_auth_middleware(AuthConfig::AwsSigV4 {
        access_key_env: ak_env.into(),
        secret_key_env: sk_env.into(),
        session_token_env: None,
        region: "us-west-2".into(),
        service: "s3".into(),
    })
    .expect("AwsSigV4 with required envs must build");
    std::env::remove_var(ak_env);
    std::env::remove_var(sk_env);
}

// ---------------------------------------------------------------------------
// Factory dispatches correctly: different configs build different strategies
// (verified by ensuring each fails on the right missing env var)
// ---------------------------------------------------------------------------

#[test]
fn test_factory_each_config_variant_fails_on_its_own_missing_env_not_others() {
    // Bearer fails on token_env, not on some Basic user_env.
    let bearer_env = "SWE_AUTH_FACTORY_DISPATCH_BRR_01";
    std::env::remove_var(bearer_env);
    // Set some Basic-looking vars that should have NO effect on Bearer.
    std::env::set_var("SWE_AUTH_FACTORY_DISPATCH_BASIC_U_01", "alice");
    std::env::set_var("SWE_AUTH_FACTORY_DISPATCH_BASIC_P_01", "pass");

    let err = AuthSvc::build_auth_middleware(AuthConfig::Bearer {
        token_env: bearer_env.into(),
    })
    .unwrap_err();
    match err {
        AuthError::MissingEnvVar { name } => assert_eq!(
            name, bearer_env,
            "Bearer must fail on its own token_env, not on unrelated vars"
        ),
        other => panic!("expected MissingEnvVar for bearer env, got {other:?}"),
    }

    std::env::remove_var("SWE_AUTH_FACTORY_DISPATCH_BASIC_U_01");
    std::env::remove_var("SWE_AUTH_FACTORY_DISPATCH_BASIC_P_01");
}
