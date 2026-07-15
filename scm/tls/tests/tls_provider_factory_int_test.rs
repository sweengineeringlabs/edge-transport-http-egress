//! Integration tests for the `TlsProviderFactory` marker trait.
//!
//! Rule 120: `src/api/identity/tls_provider_factory.rs` requires a
//! corresponding test file.
//!
//! `TlsProviderFactory` is a marker trait for TLS provider factory
//! implementations. The concrete factory behavior is exercised through
//! `HttpTlsSvc::build_tls_layer`.

use edge_transport_http_egress_tls::{HttpTlsSvc, TlsConfig};

/// @covers: TlsProviderFactory (via HttpTlsSvc)
/// The factory must produce a `TlsLayer` for `TlsConfig::None`.
#[test]
fn tls_trait_tls_provider_factory_none_config_builds_layer_int_test() {
    let result = HttpTlsSvc::build_tls_layer(TlsConfig::None);
    assert!(
        result.is_ok(),
        "TlsProviderFactory must build a layer for TlsConfig::None; got: {result:?}"
    );
}

/// @covers: TlsProviderFactory (via HttpTlsSvc — Pem missing file)
/// The factory must fail eagerly when a PEM file does not exist.
#[test]
fn tls_trait_tls_provider_factory_missing_pem_fails_eagerly_int_test() {
    let result = HttpTlsSvc::build_tls_layer(TlsConfig::Pem {
        path: "/no/such/file.pem".into(),
    });
    assert!(
        result.is_err(),
        "TlsProviderFactory must fail eagerly for missing PEM"
    );
}
