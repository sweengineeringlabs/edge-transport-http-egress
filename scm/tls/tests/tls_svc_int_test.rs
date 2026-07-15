//! Integration tests for `HttpTlsSvc` SAF functions.

use edge_transport_http_egress_tls::{HttpTlsSvc, TlsConfig};

/// @covers: HttpTlsSvc::create_config_builder
#[test]
fn test_create_config_builder_returns_builder_with_name() {
    let builder = HttpTlsSvc::create_config_builder();
    let name = builder.name();
    assert!(!name.is_empty(), "builder must carry package name");
}

/// @covers: HttpTlsSvc::build_tls_layer
#[test]
fn test_build_tls_layer_with_none_config_succeeds() {
    let result = HttpTlsSvc::build_tls_layer(TlsConfig::None);
    assert!(result.is_ok(), "build_tls_layer with None TLS must succeed");
}
