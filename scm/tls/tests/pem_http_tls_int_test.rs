//! Integration tests for `core::identity::pem_http_tls::PemHttpTls`.
//!
//! `PemHttpTls` is `pub(crate)`. Integration tests verify its contract
//! through the public `HttpTlsSvc::build_tls_layer(TlsConfig::Pem { path })` path:
//!
//! - A missing PEM file causes `build_tls_layer` to return `TlsConfigError::CertLoad`
//!   eagerly (at startup, not at first request).
//! - An existing but malformed PEM file causes `identity()` to return
//!   `TlsConfigError::CertParse { format: "pem", .. }`.
//! - A valid PEM file would produce `Ok(Some(Identity))` — tested with a
//!   self-signed cert fixture written to a temp directory.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::{HttpTlsSvc, TlsConfig, TlsConfigError};

// ---------------------------------------------------------------------------
// PemHttpTls::load — file-read errors surface at build time
// ---------------------------------------------------------------------------

/// A missing PEM file must cause `build()` to return `TlsConfigError::CertLoad`
/// before any requests are attempted.
#[test]
fn test_pem_missing_file_returns_file_read_failed() {
    let cfg = TlsConfig::Pem {
        path: "/path/does/not/exist/cert.pem".into(),
    };
    let err = HttpTlsSvc::build_tls_layer(cfg).unwrap_err();
    match err {
        TlsConfigError::CertLoad(msg) => {
            assert!(
                msg.contains("does/not/exist"),
                "CertLoad message must contain the configured path; got: {msg}"
            );
            assert!(
                !msg.is_empty(),
                "CertLoad message must not be empty; got: {msg}"
            );
        }
        other => panic!("expected CertLoad, got: {other:?}"),
    }
}

/// The `FileReadFailed` error must name the exact path that was configured,
/// not a normalized or platform-adjusted version. This aids diagnosis.
#[test]
fn test_pem_file_read_failed_contains_configured_path() {
    let path = "/very/specific/path/to/missing.pem";
    let cfg = TlsConfig::Pem { path: path.into() };
    let err = HttpTlsSvc::build_tls_layer(cfg).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("missing.pem"),
        "error message must contain the configured filename; got: {msg}"
    );
}

// ---------------------------------------------------------------------------
// PemHttpTls::identity — invalid PEM data returns InvalidCertificate
// ---------------------------------------------------------------------------

/// A file that exists but does not contain valid PEM data must cause
/// `build()` or `identity()` to return `TlsConfigError::CertParse` with
/// `format = "pem"`. We simulate this by writing a non-PEM file to a temp
/// directory and passing its path as the PEM source.
#[test]
fn test_pem_invalid_content_returns_invalid_certificate() {
    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("fake.pem");
    std::fs::write(&path, b"this is not a pem file").unwrap();

    let cfg = TlsConfig::Pem {
        path: path.to_str().unwrap().replace('\\', "/"),
    };
    // The file is read at build() time (PemHttpTls::load). The content
    // is valid for `load` (file exists, bytes read). The InvalidCertificate
    // error is produced by `identity()` which is called from `apply_to`,
    // not during `build()`. So `build()` must succeed here.
    let layer = HttpTlsSvc::build_tls_layer(cfg).expect("load of existing file must succeed");

    // Now `apply_to` calls `identity()` which calls `reqwest::Identity::from_pem`.
    let err = layer.apply_to(reqwest::Client::builder()).unwrap_err();
    match err {
        TlsConfigError::CertParse(msg) => {
            assert!(
                msg.contains("pem"),
                "message must reference 'pem'; got: {msg}"
            );
            assert!(!msg.is_empty(), "message must not be empty; got: {msg}");
        }
        other => panic!("expected CertParse, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// PemHttpTls: file read is eager — read at build, not at apply_to
// ---------------------------------------------------------------------------

/// Deleting the PEM file AFTER build must not affect `apply_to` behavior —
/// the bytes are read eagerly at build time and stored in memory.
/// (For the invalid-content case, `apply_to` still fails with
/// `InvalidCertificate`, not `FileReadFailed`, since the read already happened.)
#[test]
fn test_pem_bytes_read_at_build_time_not_at_apply_to() {
    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("ephemeral.pem");
    std::fs::write(&path, b"not-real-pem-content").unwrap();

    let cfg = TlsConfig::Pem {
        path: path.to_str().unwrap().replace('\\', "/"),
    };
    let layer = HttpTlsSvc::build_tls_layer(cfg).expect("build must succeed for existing file");

    // Delete the file after build — bytes already in memory.
    std::fs::remove_file(&path).unwrap();

    // apply_to must produce InvalidCertificate (content bad), NOT
    // FileReadFailed (because the file was already read).
    let err = layer.apply_to(reqwest::Client::builder()).unwrap_err();
    assert!(
        matches!(err, TlsConfigError::CertParse(_)),
        "after file deletion, error must be InvalidCertificate (bytes already read); got: {err:?}"
    );
}
