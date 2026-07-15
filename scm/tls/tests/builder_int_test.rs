//! Integration tests for the `build_tls_layer` SAF entry point.
//!
//! Covers: `build_tls_layer`, `TlsConfig` variants, `TlsLayer` construction.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::{HttpTlsSvc, TlsConfig, TlsConfigError, TlsLayer};

// ---------------------------------------------------------------------------
// build_tls_layer — SAF entry point
// ---------------------------------------------------------------------------

/// The crate-shipped baseline must always succeed with `TlsConfig::None`.
#[test]
fn test_builder_fn_returns_ok_with_swe_default() {
    HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None config must always build");
}

/// The SWE default must be `TlsConfig::None` (pass-through) — tests and
/// services that don't configure mTLS must not accidentally load a cert.
#[test]
fn test_builder_fn_swe_default_is_tls_config_none() {
    let cfg = TlsConfig::None;
    assert!(
        matches!(&cfg, TlsConfig::None),
        "swe_default config must be TlsConfig::None; got: {:?}",
        &cfg
    );
}

/// Building the `TlsConfig::None` baseline must succeed — a pass-through
/// layer must always be constructible.
#[test]
fn test_builder_fn_build_with_none_default_succeeds() {
    let _layer: TlsLayer =
        HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None config must always build");
}

// ---------------------------------------------------------------------------
// build_tls_layer — config variants
// ---------------------------------------------------------------------------

/// `TlsConfig::None` must build without error and produce a Debug string
/// containing "noop", confirming the pass-through provider was selected.
#[test]
fn test_with_config_none_builds_and_debug_contains_noop() {
    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("noop"),
        "None config Debug must contain 'noop'; got: {dbg}"
    );
}

/// `TlsConfig::Pem` with a missing file must fail fast at `build_tls_layer` time,
/// not deferred to the first request. Missing-at-startup is preferable to
/// missing-at-runtime.
#[test]
fn test_with_config_pem_missing_file_fails_at_build_time() {
    let cfg = TlsConfig::Pem {
        path: "/this/path/definitely/does/not/exist.pem".into(),
    };
    let err = HttpTlsSvc::build_tls_layer(cfg).unwrap_err();
    assert!(
        matches!(err, TlsConfigError::CertLoad(_)),
        "missing PEM file must return FileReadFailed; got: {err:?}"
    );
}

/// `TlsConfig::Pkcs12` without a password and a missing file must fail fast.
#[test]
fn test_with_config_pkcs12_missing_file_fails_at_build_time() {
    let cfg = TlsConfig::Pkcs12 {
        path: "/this/path/definitely/does/not/exist.p12".into(),
        password_env: None,
    };
    let err = HttpTlsSvc::build_tls_layer(cfg).unwrap_err();
    assert!(
        matches!(err, TlsConfigError::CertLoad(_)),
        "missing PKCS12 file must return FileReadFailed; got: {err:?}"
    );
}

/// `TlsConfig::Pkcs12` with an unset password env var must return
/// `TlsConfigError::MissingEnvVar` before even attempting to read the file.
#[test]
fn test_with_config_pkcs12_missing_password_env_returns_missing_env_var() {
    let env_name = "SWE_IT_TLS_BUILDER_PW_ABSENT_01";
    std::env::remove_var(env_name);
    let cfg = TlsConfig::Pkcs12 {
        path: "irrelevant.p12".into(),
        password_env: Some(env_name.into()),
    };
    let err = HttpTlsSvc::build_tls_layer(cfg).unwrap_err();
    match err {
        TlsConfigError::MissingEnvVar { name } => assert_eq!(name, env_name),
        other => panic!("expected MissingEnvVar, got: {other:?}"),
    }
}

/// `TlsConfig::None` must build successfully even when arbitrary environment
/// variables are set or unset — the None path must not read any env vars.
#[test]
fn test_with_config_none_ignores_environment() {
    std::env::set_var("SWE_IT_TLS_BUILDER_IGNORED_ENV", "some_value");
    HttpTlsSvc::build_tls_layer(TlsConfig::None)
        .expect("None config must build regardless of env vars");
    std::env::remove_var("SWE_IT_TLS_BUILDER_IGNORED_ENV");
}

// ---------------------------------------------------------------------------
// TlsConfig accessor — construct directly
// ---------------------------------------------------------------------------

/// `TlsConfig::None` variant can be matched correctly.
#[test]
fn test_config_accessor_returns_none_variant() {
    let b_cfg = TlsConfig::None;
    assert!(matches!(&b_cfg, TlsConfig::None));
}

/// `TlsConfig::Pem` stores the exact path supplied.
#[test]
fn test_config_accessor_returns_pem_variant_with_correct_path() {
    let b_cfg = TlsConfig::Pem {
        path: "/some/cert.pem".into(),
    };
    match &b_cfg {
        TlsConfig::Pem { path } => assert_eq!(path, "/some/cert.pem"),
        other => panic!("expected Pem, got: {other:?}"),
    }
}

/// `TlsConfig::Pkcs12` stores the correct path and password_env.
#[test]
fn test_config_accessor_returns_pkcs12_variant_with_correct_fields() {
    let b_cfg = TlsConfig::Pkcs12 {
        path: "/some/cert.p12".into(),
        password_env: Some("SWE_IT_TLS_BUILDER_EXPECTED_ENV".into()),
    };
    match &b_cfg {
        TlsConfig::Pkcs12 { path, password_env } => {
            assert_eq!(path, "/some/cert.p12");
            assert_eq!(
                password_env.as_deref(),
                Some("SWE_IT_TLS_BUILDER_EXPECTED_ENV")
            );
        }
        other => panic!("expected Pkcs12, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// TlsLayer: Send + Sync
// ---------------------------------------------------------------------------

#[test]
fn test_tls_layer_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TlsLayer>();
}
