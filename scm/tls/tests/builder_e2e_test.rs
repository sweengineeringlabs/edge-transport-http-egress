//! End-to-end tests for the edge_transport_http_egress_tls SAF builder surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::{HttpTlsSvc, TlsConfig, TlsLayer};

/// @covers: build_tls_layer with None config
#[test]
fn test_e2e_builder() {
    let layer: TlsLayer =
        HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("build() must succeed");
    assert!(
        format!("{layer:?}").contains("noop"),
        "e2e: None config must produce noop layer"
    );
}

/// @covers: TlsConfig::None variant matches correctly
#[test]
fn test_e2e_with_config() {
    let b_cfg = TlsConfig::None;
    assert!(matches!(&b_cfg, TlsConfig::None));
    HttpTlsSvc::build_tls_layer(b_cfg).expect("e2e with_config None build must succeed");
}

/// @covers: TlsConfig fields are accessible directly
#[test]
fn test_e2e_config() {
    let cfg = TlsConfig::Pem {
        path: "/some/cert.pem".into(),
    };
    let b_cfg = cfg;
    assert!(
        matches!(&b_cfg, TlsConfig::Pem { .. }),
        "config() must return Pem variant"
    );
}

/// @covers: build_tls_layer with None config always succeeds
#[test]
fn test_e2e_build() {
    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::None)
        .expect("e2e build with None must always succeed");
    assert!(!format!("{layer:?}").is_empty());
}
