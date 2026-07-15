//! Integration tests for `api/error.rs` — the public `RateError` enum.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::RateError;

// ---------------------------------------------------------------------------
// RateError::ParseFailed
// ---------------------------------------------------------------------------

/// `ParseFailed` must be publicly constructable.
#[test]
fn test_error_parse_failed_is_publicly_constructable() {
    let _err = RateError::ParseFailed("any reason".to_string());
}

/// `Display` must name the crate.
#[test]
fn test_error_parse_failed_display_names_crate() {
    let err = RateError::ParseFailed("bad field".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("edge_transport_http_egress_rate"),
        "ParseFailed display must name the crate; got: {msg}"
    );
}

/// `Display` must echo the supplied reason.
#[test]
fn test_error_parse_failed_display_echoes_reason() {
    let reason = "missing field `tokens_per_second`";
    let err = RateError::ParseFailed(reason.to_string());
    assert!(
        err.to_string().contains(reason),
        "ParseFailed display must contain the reason; got: {}",
        err
    );
}

/// Empty reason still produces a non-empty display.
#[test]
fn test_error_parse_failed_with_empty_reason_display_is_non_empty() {
    let err = RateError::ParseFailed(String::new());
    assert!(
        !err.to_string().is_empty(),
        "ParseFailed display must not be empty even with empty reason"
    );
}

// ---------------------------------------------------------------------------
// Debug
// ---------------------------------------------------------------------------

/// `ParseFailed` must implement `Debug`.
#[test]
fn test_error_variants_implement_debug() {
    let _ = format!("{:?}", RateError::ParseFailed("p".to_string()));
}
