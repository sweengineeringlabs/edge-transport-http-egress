//! Integration tests for the [`Validator`] trait contract.
//!
//! Verifies that the SAF `validate` and `validate_http_config` functions
//! correctly enforce the `Validator` contract: `Ok(())` for well-formed values
//! and `Err(msg)` with a non-empty, actionable message for invalid ones.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{HttpConfig, HttpTransportSvc};

// ─── validate_http_config tests ─────────────────────────────────────────────

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_returns_ok_for_default_config() {
    let cfg = HttpConfig::default();
    assert!(
        HttpTransportSvc::validate_http_config(&cfg).is_ok(),
        "default HttpConfig must pass validation"
    );
}

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_returns_err_for_zero_timeout_secs() {
    let cfg = HttpConfig {
        timeout_secs: 0,
        ..HttpConfig::default()
    };
    let err = HttpTransportSvc::validate_http_config(&cfg).unwrap_err();
    assert!(
        !err.is_empty(),
        "validation error message must be non-empty"
    );
    assert!(
        err.contains("timeout_secs"),
        "error must name the offending field, got: {err:?}"
    );
}

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_returns_err_for_zero_connect_timeout() {
    let cfg = HttpConfig {
        connect_timeout_secs: 0,
        ..HttpConfig::default()
    };
    let err = HttpTransportSvc::validate_http_config(&cfg).unwrap_err();
    assert!(
        !err.is_empty(),
        "validation error message must be non-empty"
    );
    assert!(
        err.contains("connect_timeout_secs"),
        "error must name the offending field, got: {err:?}"
    );
}

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_with_base_url_returns_ok() {
    let cfg = HttpConfig::with_base_url("https://api.example.com");
    assert!(
        HttpTransportSvc::validate_http_config(&cfg).is_ok(),
        "HttpConfig with base_url and defaults must pass validation"
    );
}
