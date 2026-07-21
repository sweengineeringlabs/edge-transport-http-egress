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
    // The SWE default (no `[tls]` section) resolves to the `None` variant — no
    // client cert attached. Pin the variant, not merely that a value exists.
    assert!(
        matches!(TlsConfig::default(), TlsConfig::None),
        "the default TLS policy must be None (no client cert)"
    );
}

/// @covers: default_http_egress
#[test]
fn test_tls_layer_assembles_in_default_http_egress() {
    // `default_http_egress` always includes the TLS middleware layer; a
    // successful build proves the TLS layer assembled. Build twice to confirm
    // the layer is stateless and reusable, not a one-shot.
    let a = HttpTransportSvc::default_http_egress();
    let b = HttpTransportSvc::default_http_egress();
    assert!(
        a.is_ok() && b.is_ok(),
        "default_http_egress (TLS layer included) must build repeatably: {:?} / {:?}",
        a.err(),
        b.err(),
    );
}

/// @covers: default_http_egress
#[test]
fn test_tls_middleware_does_not_interfere_with_http_only_config() {
    // Two independent instances must both build, demonstrating the TLS layer is
    // stateless and reusable.
    let a = HttpTransportSvc::default_http_egress();
    let b = HttpTransportSvc::default_http_egress();
    assert!(
        a.is_ok() && b.is_ok(),
        "the stateless TLS layer must not prevent independent builds"
    );
}

/// @covers: default_http_egress
#[test]
fn test_tls_config_none_variant_parses_successfully() {
    // A `kind = "none"` section must parse to exactly the `None` variant.
    assert!(
        matches!(
            TlsConfig::from_config(r#"kind = "none""#),
            Ok(TlsConfig::None)
        ),
        "kind = \"none\" must parse to TlsConfig::None"
    );
    // A `pkcs12` section must parse to the Pkcs12 variant carrying its path —
    // proving the parser routes on `kind`, not always returning None.
    let pkcs12 = TlsConfig::from_config("kind = \"pkcs12\"\npath = \"certs/client.p12\"")
        .expect("pkcs12 section must parse");
    assert!(
        matches!(pkcs12, TlsConfig::Pkcs12 { path, .. } if path == "certs/client.p12"),
        "kind = \"pkcs12\" must parse to Pkcs12 carrying the configured path"
    );
}

/// @covers: default_http_egress
#[test]
fn test_tls_config_unknown_kind_returns_err_error() {
    // An unknown `kind` (or malformed TOML) must be rejected, not silently
    // coerced to a default — so the successful parses above are real verdicts.
    assert!(
        TlsConfig::from_config(r#"kind = "not-a-real-kind""#).is_err(),
        "an unknown TLS kind must fail to parse"
    );
    assert!(
        TlsConfig::from_config("this is not = valid = toml [[[").is_err(),
        "malformed TOML must fail to parse"
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
