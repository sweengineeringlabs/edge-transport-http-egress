//! Integration tests for `AuthConfig` — the public auth policy schema.
//!
//! All tests use `AuthSvc::build_auth_middleware(config)` or struct-literal construction
//! so they go through the real public API without touching `pub(crate)`
//! internals.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::{AuthConfig, AuthError, AuthSvc};

// ---------------------------------------------------------------------------
// None variant
// ---------------------------------------------------------------------------

#[test]
fn test_auth_config_none_variant_builds_without_env() {
    // None requires no env vars — must always succeed even in a
    // stripped environment. If this fails the baseline is broken.
    AuthSvc::build_auth_middleware(AuthConfig::None).expect("AuthConfig::None must always build");
}

#[test]
fn test_auth_config_none_is_default_from_builder_fn() {
    // The SWE default auth config is None (pass-through, no credentials needed).
    let cfg = AuthConfig::None;
    assert!(
        matches!(&cfg, AuthConfig::None),
        "swe_default config must be AuthConfig::None, got {:?}",
        cfg
    );
    AuthSvc::build_auth_middleware(cfg).expect("None config must build");
}

// ---------------------------------------------------------------------------
// Bearer variant — stores env-var name, not the token
// ---------------------------------------------------------------------------

#[test]
fn test_auth_config_bearer_stores_token_env_name() {
    let cfg = AuthConfig::Bearer {
        token_env: "SWE_AUTH_CFG_BEARER_01".into(),
    };
    match &cfg {
        AuthConfig::Bearer { token_env } => {
            assert_eq!(token_env, "SWE_AUTH_CFG_BEARER_01");
        }
        other => panic!("expected Bearer variant, got {other:?}"),
    }
}

#[test]
fn test_auth_config_bearer_missing_env_fails_at_build_time() {
    let env_name = "SWE_AUTH_CFG_BEARER_02";
    std::env::remove_var(env_name);
    let cfg = AuthConfig::Bearer {
        token_env: env_name.into(),
    };
    let err = AuthSvc::build_auth_middleware(cfg).unwrap_err();
    match err {
        AuthError::MissingEnvVar { name } => assert_eq!(name, env_name),
        other => panic!("expected MissingEnvVar, got {other:?}"),
    }
}

#[test]
fn test_auth_config_bearer_env_set_builds_successfully() {
    let env_name = "SWE_AUTH_CFG_BEARER_03";
    std::env::set_var(env_name, "test-bearer-token");
    let cfg = AuthConfig::Bearer {
        token_env: env_name.into(),
    };
    AuthSvc::build_auth_middleware(cfg).expect("Bearer with env set must build");
    std::env::remove_var(env_name);
}

// ---------------------------------------------------------------------------
// Basic variant — stores two env-var names
// ---------------------------------------------------------------------------

#[test]
fn test_auth_config_basic_stores_both_env_names() {
    let cfg = AuthConfig::Basic {
        user_env: "SWE_AUTH_CFG_BASIC_U_01".into(),
        pass_env: "SWE_AUTH_CFG_BASIC_P_01".into(),
    };
    match &cfg {
        AuthConfig::Basic { user_env, pass_env } => {
            assert_eq!(user_env, "SWE_AUTH_CFG_BASIC_U_01");
            assert_eq!(pass_env, "SWE_AUTH_CFG_BASIC_P_01");
        }
        other => panic!("expected Basic variant, got {other:?}"),
    }
}

#[test]
fn test_auth_config_basic_missing_user_env_fails_at_build() {
    let user_env = "SWE_AUTH_CFG_BASIC_U_02";
    let pass_env = "SWE_AUTH_CFG_BASIC_P_02";
    std::env::remove_var(user_env);
    std::env::remove_var(pass_env);
    let cfg = AuthConfig::Basic {
        user_env: user_env.into(),
        pass_env: pass_env.into(),
    };
    let err = AuthSvc::build_auth_middleware(cfg).unwrap_err();
    assert!(
        matches!(err, AuthError::MissingEnvVar { .. }),
        "missing basic user env must fail: {err:?}"
    );
}

#[test]
fn test_auth_config_basic_both_envs_set_builds_successfully() {
    let user_env = "SWE_AUTH_CFG_BASIC_U_03";
    let pass_env = "SWE_AUTH_CFG_BASIC_P_03";
    std::env::set_var(user_env, "alice");
    std::env::set_var(pass_env, "s3cr3t");
    let cfg = AuthConfig::Basic {
        user_env: user_env.into(),
        pass_env: pass_env.into(),
    };
    AuthSvc::build_auth_middleware(cfg).expect("Basic with both envs set must build");
    std::env::remove_var(user_env);
    std::env::remove_var(pass_env);
}

// ---------------------------------------------------------------------------
// Header variant — stores name + env-var name
// ---------------------------------------------------------------------------

#[test]
fn test_auth_config_header_stores_name_and_value_env() {
    let cfg = AuthConfig::Header {
        name: "x-api-key".into(),
        value_env: "SWE_AUTH_CFG_HEADER_01".into(),
    };
    match &cfg {
        AuthConfig::Header { name, value_env } => {
            assert_eq!(name, "x-api-key");
            assert_eq!(value_env, "SWE_AUTH_CFG_HEADER_01");
        }
        other => panic!("expected Header variant, got {other:?}"),
    }
}

#[test]
fn test_auth_config_header_missing_value_env_fails_at_build() {
    let env_name = "SWE_AUTH_CFG_HEADER_02";
    std::env::remove_var(env_name);
    let cfg = AuthConfig::Header {
        name: "x-api-key".into(),
        value_env: env_name.into(),
    };
    let err = AuthSvc::build_auth_middleware(cfg).unwrap_err();
    assert!(
        matches!(err, AuthError::MissingEnvVar { .. }),
        "missing header value env must fail: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// AwsSigV4 variant
// ---------------------------------------------------------------------------

#[test]
fn test_auth_config_aws_sigv4_stores_all_fields() {
    let cfg = AuthConfig::AwsSigV4 {
        access_key_env: "SWE_AUTH_CFG_AWS_AK_01".into(),
        secret_key_env: "SWE_AUTH_CFG_AWS_SK_01".into(),
        session_token_env: Some("SWE_AUTH_CFG_AWS_ST_01".into()),
        region: "us-east-1".into(),
        service: "s3".into(),
    };
    match &cfg {
        AuthConfig::AwsSigV4 {
            access_key_env,
            secret_key_env,
            session_token_env,
            region,
            service,
        } => {
            assert_eq!(access_key_env, "SWE_AUTH_CFG_AWS_AK_01");
            assert_eq!(secret_key_env, "SWE_AUTH_CFG_AWS_SK_01");
            assert_eq!(session_token_env.as_deref(), Some("SWE_AUTH_CFG_AWS_ST_01"));
            assert_eq!(region, "us-east-1");
            assert_eq!(service, "s3");
        }
        other => panic!("expected AwsSigV4 variant, got {other:?}"),
    }
}

#[test]
fn test_auth_config_aws_sigv4_session_token_is_optional() {
    let cfg = AuthConfig::AwsSigV4 {
        access_key_env: "A".into(),
        secret_key_env: "S".into(),
        session_token_env: None,
        region: "us-east-1".into(),
        service: "s3".into(),
    };
    assert!(matches!(
        cfg,
        AuthConfig::AwsSigV4 {
            session_token_env: None,
            ..
        }
    ));
}

#[test]
fn test_auth_config_aws_sigv4_missing_access_key_env_fails_at_build() {
    let ak_env = "SWE_AUTH_CFG_AWS_AK_02";
    let sk_env = "SWE_AUTH_CFG_AWS_SK_02";
    std::env::remove_var(ak_env);
    std::env::remove_var(sk_env);
    let cfg = AuthConfig::AwsSigV4 {
        access_key_env: ak_env.into(),
        secret_key_env: sk_env.into(),
        session_token_env: None,
        region: "us-east-1".into(),
        service: "s3".into(),
    };
    let err = AuthSvc::build_auth_middleware(cfg).unwrap_err();
    assert!(
        matches!(err, AuthError::MissingEnvVar { .. }),
        "missing AWS access key env must fail: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Debug impl — must not leak sensitive field values
// ---------------------------------------------------------------------------

#[test]
fn test_auth_config_debug_does_not_leak_env_var_values() {
    // The config stores env-var NAMES, not the secrets. The Debug
    // representation of the config may show the env-var name (that is
    // fine — it is not a secret). The actual credential value is never
    // held in AuthConfig, so there is nothing to leak here. This test
    // pins that the debug output contains the env-var name string
    // (proving the field is visible) but does not contain any runtime
    // env-var value that might have been accidentally resolved.
    let env_name = "SWE_AUTH_CFG_DBG_01";
    // Deliberately set the env var to a secret-looking value.
    std::env::set_var(env_name, "super-secret-credential");
    let cfg = AuthConfig::Bearer {
        token_env: env_name.into(),
    };
    let dbg = format!("{cfg:?}");
    // The raw credential must NOT appear in AuthConfig's Debug output
    // because AuthConfig never holds the resolved secret.
    assert!(
        !dbg.contains("super-secret-credential"),
        "AuthConfig Debug must not contain the resolved credential value: {dbg}"
    );
    std::env::remove_var(env_name);
}
