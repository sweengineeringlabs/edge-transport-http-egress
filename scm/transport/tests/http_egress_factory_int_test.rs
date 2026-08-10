//! Integration tests for the bare-primitive SAF factory functions.
//!
//! Covers: `plain_http_egress`, `plain_http_stream`, and `validate_http_config`.
//! (Composed constructors moved to the composition root — see
//! edge-transport-http-egress#28 / edge-bootstrap ADR-008.)

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{HttpConfig, HttpStream, HttpTransportSvc};

// ─── plain_http_egress ──────────────────────────────────────────────────────

/// @covers: plain_http_egress
#[test]
fn test_plain_http_egress_builds_with_default_config() {
    // Opaque `Box<dyn HttpEgress>` return; assert the factory is repeatably
    // callable. End-to-end send behaviour is covered in reqwest_middleware_int_test.rs.
    let a = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(
        a.is_ok() && b.is_ok(),
        "plain_http_egress must build repeatably with default config: {:?} / {:?}",
        a.err(),
        b.err(),
    );
}

/// @covers: plain_http_egress
#[test]
fn test_plain_http_egress_builds_with_custom_base_url() {
    let with_url =
        HttpTransportSvc::plain_http_egress(HttpConfig::with_base_url("https://custom.api.com"));
    let with_header =
        HttpTransportSvc::plain_http_egress(HttpConfig::default().with_header("x-env", "test"));
    assert!(
        with_url.is_ok() && with_header.is_ok(),
        "plain_http_egress must build with custom base URL and with custom headers: {:?} / {:?}",
        with_url.err(),
        with_header.err(),
    );
}

// ─── plain_http_stream ────────────────────────────────────────────────────────

/// @covers: plain_http_stream
#[test]
fn test_plain_http_stream_builds_with_default_config() {
    let a = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    let b = HttpTransportSvc::plain_http_stream(HttpConfig::default());
    assert!(
        a.is_ok() && b.is_ok(),
        "plain_http_stream must build repeatably: {:?} / {:?}",
        a.err(),
        b.err(),
    );
}

/// @covers: plain_http_stream
#[test]
fn test_plain_http_stream_implements_stream_trait() {
    let outbound = HttpTransportSvc::plain_http_stream(HttpConfig::default()).unwrap();
    fn _assert(_: &dyn HttpStream) {}
    _assert(outbound.as_ref());
}

// ─── validate_http_config ─────────────────────────────────────────────────────

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_returns_ok_for_valid_timeout() {
    let cfg = HttpConfig {
        timeout_secs: 30,
        connect_timeout_secs: 10,
        ..HttpConfig::default()
    };
    assert!(
        HttpTransportSvc::validate_http_config(&cfg).is_ok(),
        "a config with positive timeouts must validate"
    );
    // Sibling negative: dropping the request timeout to zero must fail, so the
    // Ok above genuinely inspects the timeout fields.
    let bad = HttpConfig {
        timeout_secs: 0,
        connect_timeout_secs: 10,
        ..HttpConfig::default()
    };
    assert!(
        HttpTransportSvc::validate_http_config(&bad).is_err(),
        "a zero request timeout must fail validation"
    );
}

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_returns_err_for_zero_timeout() {
    let cfg = HttpConfig {
        timeout_secs: 0,
        ..HttpConfig::default()
    };
    let err = HttpTransportSvc::validate_http_config(&cfg).unwrap_err();
    assert!(
        err.contains("timeout_secs"),
        "error must name the offending field, got: {err:?}"
    );
}
