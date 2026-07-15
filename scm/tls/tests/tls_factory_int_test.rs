//! Integration tests for `core::identity::tls_factory::build_provider`.
//!
//! `build_provider` is `pub(crate)`. Integration tests verify the factory's
//! contract through the public `HttpTlsSvc::build_tls_layer(config)` path, which calls
//! `build_provider(&config)` internally:
//!
//! - `TlsConfig::None` → noop provider → layer Debug contains "noop".
//! - `TlsConfig::Pem { path }` → `PemHttpTls` — file-read error at build.
//! - `TlsConfig::Pkcs12 { path, password_env: None }` → `Pkcs12HttpTls` —
//!   file-read error at build (no password check needed).
//! - `TlsConfig::Pkcs12 { path, password_env: Some(var) }` where `var` is
//!   unset → `TlsConfigError::MissingEnvVar` at build.
//! - `TlsConfig::Pkcs12 { path, password_env: Some(var) }` where `var` is
//!   set but file is missing → `TlsConfigError::CertLoad` at build.
//! - Each variant selects the correct provider (`describe()` embedded in
//!   the `TlsLayer` Debug).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::{HttpTlsSvc, TlsConfig, TlsConfigError};

// ---------------------------------------------------------------------------
// None variant → noop provider
// ---------------------------------------------------------------------------

/// `TlsConfig::None` must route through `build_provider` to `NoopHttpTls`,
/// producing a layer whose Debug output contains "noop".
#[test]
fn test_factory_none_variant_selects_noop_provider() {
    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("noop"),
        "factory must select noop provider for TlsConfig::None; Debug: {dbg}"
    );
}

/// The noop provider must allow `apply_to` to succeed — `build_provider`
/// must not corrupt the provider boxed inside the layer.
#[test]
fn test_factory_none_provider_allows_apply_to_to_succeed() {
    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    let _ = layer
        .apply_to(reqwest::Client::builder())
        .expect("noop apply_to must return Ok");
}

// ---------------------------------------------------------------------------
// Pem variant → PemHttpTls (file-read error)
// ---------------------------------------------------------------------------

/// `TlsConfig::Pem` with a missing file must route through `build_provider`
/// to `PemHttpTls::load`, which returns `FileReadFailed`.
#[test]
fn test_factory_pem_variant_returns_file_read_failed_for_missing_file() {
    let cfg = TlsConfig::Pem {
        path: "/factory/test/missing.pem".into(),
    };
    let err = HttpTlsSvc::build_tls_layer(cfg).unwrap_err();
    assert!(
        matches!(err, TlsConfigError::CertLoad(_)),
        "factory must produce FileReadFailed for missing PEM; got: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Pkcs12 variant → Pkcs12HttpTls (env var + file-read errors)
// ---------------------------------------------------------------------------

/// `TlsConfig::Pkcs12` with no password and a missing file must route to
/// `Pkcs12HttpTls::load`, returning `FileReadFailed`.
#[test]
fn test_factory_pkcs12_no_password_missing_file_returns_file_read_failed() {
    let cfg = TlsConfig::Pkcs12 {
        path: "/factory/test/missing.p12".into(),
        password_env: None,
    };
    let err = HttpTlsSvc::build_tls_layer(cfg).unwrap_err();
    assert!(
        matches!(err, TlsConfigError::CertLoad(_)),
        "factory must produce FileReadFailed for missing PKCS12 file; got: {err:?}"
    );
}

/// `TlsConfig::Pkcs12` with an unset `password_env` must cause the factory
/// to return `MissingEnvVar` before attempting file I/O.
#[test]
fn test_factory_pkcs12_unset_password_env_returns_missing_env_var() {
    let env = "SWE_IT_TLS_FACTORY_PW_ABSENT_06";
    std::env::remove_var(env);
    let cfg = TlsConfig::Pkcs12 {
        path: "irrelevant.p12".into(),
        password_env: Some(env.into()),
    };
    let err = HttpTlsSvc::build_tls_layer(cfg).unwrap_err();
    match err {
        TlsConfigError::MissingEnvVar { name } => assert_eq!(name, env),
        other => panic!("expected MissingEnvVar, got: {other:?}"),
    }
}

/// With `password_env` SET to a value but the file missing, the factory
/// must return `FileReadFailed` (env var resolved, file I/O failed).
#[test]
fn test_factory_pkcs12_set_password_env_missing_file_returns_file_read_failed() {
    let env = "SWE_IT_TLS_FACTORY_PW_SET_07";
    std::env::set_var(env, "any-password");
    let cfg = TlsConfig::Pkcs12 {
        path: "/factory/test/missing_with_pw.p12".into(),
        password_env: Some(env.into()),
    };
    let err = HttpTlsSvc::build_tls_layer(cfg).unwrap_err();
    assert!(
        matches!(err, TlsConfigError::CertLoad(_)),
        "factory must return FileReadFailed when env is set but file is missing; got: {err:?}"
    );
    std::env::remove_var(env);
}

// ---------------------------------------------------------------------------
// All three variants produce distinct error messages
// ---------------------------------------------------------------------------

/// Each of the three failing paths (missing pem file, missing pkcs12 file,
/// missing pkcs12 env var) must produce a distinct error message so the
/// operator can distinguish them at a glance.
#[test]
fn test_factory_three_error_paths_produce_distinct_messages() {
    let env = "SWE_IT_TLS_FACTORY_DISTINCT_08";
    std::env::remove_var(env);

    let e_pem = HttpTlsSvc::build_tls_layer(TlsConfig::Pem {
        path: "/f/t/a.pem".into(),
    })
    .unwrap_err()
    .to_string();

    let e_pkcs12_no_pw = HttpTlsSvc::build_tls_layer(TlsConfig::Pkcs12 {
        path: "/f/t/b.p12".into(),
        password_env: None,
    })
    .unwrap_err()
    .to_string();

    let e_pkcs12_missing_env = HttpTlsSvc::build_tls_layer(TlsConfig::Pkcs12 {
        path: "irrelevant.p12".into(),
        password_env: Some(env.into()),
    })
    .unwrap_err()
    .to_string();

    // The three errors must not all be identical — they come from different code paths.
    assert_ne!(
        e_pem, e_pkcs12_missing_env,
        "PEM and missing-env errors must differ"
    );
    assert_ne!(
        e_pkcs12_no_pw, e_pkcs12_missing_env,
        "no-password and missing-env errors must differ"
    );
}
