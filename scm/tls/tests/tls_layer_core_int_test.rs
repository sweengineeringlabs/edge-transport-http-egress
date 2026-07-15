//! Integration tests for `core::tls_layer` — the `TlsLayer` impl block
//! containing `new` and `apply_to`.
//!
//! All core methods are `pub(crate)` or `pub`. `apply_to` is declared `pub`
//! on `TlsLayer` directly in `core::tls_layer`. Integration tests exercise:
//!
//! - `apply_to` for `TlsConfig::None` (noop → Ok, builder unchanged).
//! - `apply_to` for invalid cert data (non-None → InvalidCertificate).
//! - `apply_to` idempotency (safe to call multiple times).
//! - The `TlsLayer` Debug output reflects the provider.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::{HttpTlsSvc, TlsConfig, TlsConfigError, TlsLayer};

// ---------------------------------------------------------------------------
// TlsLayer::apply_to — TlsConfig::None (noop path)
// ---------------------------------------------------------------------------

/// `apply_to` with a noop `TlsLayer` must return `Ok` and leave the
/// `ClientBuilder` in a state that can successfully call `build()`.
#[test]
fn test_apply_to_none_returns_buildable_client_builder() {
    let layer: TlsLayer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    let cb = layer
        .apply_to(reqwest::Client::builder())
        .expect("apply_to None must return Ok");
    let _client = cb
        .build()
        .expect("ClientBuilder must build after noop apply_to");
}

/// `apply_to` with a noop layer must be callable multiple times without error.
/// The provider is behind `Arc` so it is not consumed.
#[test]
fn test_apply_to_none_is_idempotent() {
    let layer: TlsLayer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    for i in 0..3 {
        let _ = layer
            .apply_to(reqwest::Client::builder())
            .unwrap_or_else(|e| panic!("apply_to call {i} must succeed; got: {e:?}"));
    }
}

// ---------------------------------------------------------------------------
// TlsLayer::apply_to — invalid cert (PEM) returns InvalidCertificate
// ---------------------------------------------------------------------------

/// `apply_to` with a layer built from an existing but malformed PEM file
/// must return `TlsConfigError::CertParse { format: "pem", .. }`.
#[test]
fn test_apply_to_pem_invalid_content_returns_invalid_certificate() {
    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("invalid.pem");
    std::fs::write(&path, b"not-a-pem-file").unwrap();

    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::Pem {
        path: path.to_str().unwrap().replace('\\', "/"),
    })
    .expect("file exists, build must succeed");

    let err = layer.apply_to(reqwest::Client::builder()).unwrap_err();
    match err {
        TlsConfigError::CertParse(msg) => {
            assert!(
                msg.contains("pem"),
                "message must reference 'pem'; got: {msg}"
            );
        }
        other => panic!("expected CertParse, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// TlsLayer::apply_to — invalid cert (PKCS12) returns InvalidCertificate
// ---------------------------------------------------------------------------

/// `apply_to` with a layer built from an existing but malformed PKCS12
/// file must return `TlsConfigError::CertParse` containing "pkcs12".
#[test]
fn test_apply_to_pkcs12_invalid_content_returns_invalid_certificate() {
    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("invalid.p12");
    std::fs::write(&path, b"not-pkcs12-content").unwrap();

    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::Pkcs12 {
        path: path.to_str().unwrap().replace('\\', "/"),
        password_env: None,
    })
    .expect("file exists, build must succeed");

    let err = layer.apply_to(reqwest::Client::builder()).unwrap_err();
    match err {
        TlsConfigError::CertParse(msg) => {
            assert!(
                msg.contains("pkcs12"),
                "message must reference 'pkcs12'; got: {msg}"
            );
        }
        other => panic!("expected CertParse, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// TlsLayer Debug — reflects provider
// ---------------------------------------------------------------------------

/// The `TlsLayer` Debug output must contain "TlsLayer" and the provider
/// name so operators can identify the active identity configuration.
#[test]
fn test_tls_layer_debug_contains_struct_name_and_provider() {
    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("TlsLayer"),
        "Debug must name the struct; got: {dbg}"
    );
    assert!(
        dbg.contains("noop"),
        "Debug must name the provider; got: {dbg}"
    );
}

// ---------------------------------------------------------------------------
// TlsLayer: Send + Sync
// ---------------------------------------------------------------------------

#[test]
fn test_core_tls_layer_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TlsLayer>();
}

/// `TlsLayer` must be usable across thread boundaries via `std::thread::spawn`.
#[test]
fn test_core_tls_layer_send_across_thread() {
    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    let handle = std::thread::spawn(move || {
        layer
            .apply_to(reqwest::Client::builder())
            .expect("apply_to in thread")
            .build()
            .expect("build in thread")
    });
    handle.join().expect("thread must not panic");
}
