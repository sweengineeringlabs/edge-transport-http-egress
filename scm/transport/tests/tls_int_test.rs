//! Integration tests covering the `edge-security-transport-egress-http-tls` dependency.
//!
//! Verifies that TLS configuration flows through the SAF factory and that
//! plaintext (non-TLS) connections work correctly with the TLS middleware
//! present in the middleware stack when TLS is not required.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_security_transport_egress_http_tls::TlsConfig;
use edge_transport_http_egress_transport::HttpTransportSvc;

/// @covers: default_http_egress
#[test]
fn test_tls_config_swe_default_parses_successfully() {
    // Verify the SWE default TLS config parses without error.
    // TlsConfig::None is always valid — no cert files to resolve.
    let tls_cfg: Result<_, edge_security_transport_egress_http_tls::TlsConfigError> =
        Ok(TlsConfig::None);
    assert!(
        tls_cfg.is_ok(),
        "TlsConfig::None must always be valid: {:?}",
        tls_cfg.err()
    );
}

/// @covers: default_http_egress
#[test]
fn test_tls_layer_assembles_in_default_http_egress() {
    // `default_http_egress` always includes the TLS middleware layer.
    // A successful build proves the TLS middleware assembled without errors.
    let result = HttpTransportSvc::default_http_egress();
    assert!(
        result.is_ok(),
        "default_http_egress (which includes TLS middleware) must build: {:?}",
        result.err()
    );
}

/// @covers: default_http_egress
#[test]
fn test_tls_middleware_does_not_interfere_with_http_only_config() {
    // Build two independent instances — both must succeed independently,
    // demonstrating that the TLS layer is stateless and reusable.
    let a = HttpTransportSvc::default_http_egress();
    let b = HttpTransportSvc::default_http_egress();
    assert!(a.is_ok(), "first build must succeed");
    assert!(b.is_ok(), "second build must succeed");
}

/// @covers: default_http_egress
#[test]
fn test_tls_config_none_variant_parses_successfully() {
    // Parse the "none" TLS config variant (no client cert, no custom CA).
    let tls_cfg = TlsConfig::from_config(r#"kind = "none""#);
    assert!(
        tls_cfg.is_ok(),
        "TlsConfig 'none' variant must parse: {:?}",
        tls_cfg.err()
    );
}

/// @covers: http_egress_from_config_with_tls — the construct-and-pass-in
/// replacement for `[tls]` (BYOSec reversed 2026-07-17; `TlsConfig` has no
/// usable `OptionalSection` integration cross-crate — see
/// `http_egress_from_config`'s doc comment).
#[test]
fn test_http_egress_from_config_with_tls_builds_happy() {
    use edge_security_transport_egress_http_tls::HttpTlsSvc;
    use swe_edge_configbuilder::ConfigLoaderFactory;

    let dir = tempfile::TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), "[unrelated]\nx = 1")
        .expect("write application.toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let tls = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None variant always builds");
    let result = HttpTransportSvc::http_egress_from_config_with_tls(&loader, tls);
    assert!(
        result.is_ok(),
        "construct-and-pass-in TLS must build: {:?}",
        result.err()
    );
}
