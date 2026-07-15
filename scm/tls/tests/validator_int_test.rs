//! Integration tests for `validate_tls_config` — `Validator` contract via SAF wrapper.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::{validate_tls_config, TlsConfig};

/// @covers: validate_tls_config
#[test]
fn tls_trait_validator_validate_returns_ok_for_none_int_test() {
    assert!(validate_tls_config(&TlsConfig::None).is_ok());
}

/// @covers: validate_tls_config
#[test]
fn tls_trait_validator_validate_returns_err_for_empty_pem_path_int_test() {
    let cfg = TlsConfig::Pem {
        path: "".to_string(),
    };
    let err = validate_tls_config(&cfg).unwrap_err();
    assert!(!err.is_empty(), "error message must be non-empty");
}

/// @covers: validate_tls_config
#[test]
fn tls_trait_validator_validate_returns_err_for_empty_pkcs12_path_int_test() {
    let cfg = TlsConfig::Pkcs12 {
        path: "".to_string(),
        password_env: None,
    };
    let err = validate_tls_config(&cfg).unwrap_err();
    assert!(!err.is_empty(), "error message must be non-empty");
}
