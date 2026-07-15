#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `edge_transport_http_egress_retry::RetryError`.
//!
//! Covers: `RetryRetryError::ParseFailed` — Display messages
//! must be actionable: name the crate, embed the payload, be non-empty.

use edge_transport_http_egress_retry::RetryError;

// ---------------------------------------------------------------------------
// RetryError::ParseFailed
// ---------------------------------------------------------------------------

/// `ParseFailed` display must name the crate so the error is traceable.
#[test]
fn test_parse_failed_display_names_the_crate() {
    let err = RetryError::ParseFailed("unexpected field `max_retry`".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("edge_transport_http_egress_retry"),
        "ParseFailed Display must name the crate; got: {msg}"
    );
}

/// `ParseFailed` display must echo the supplied reason so the operator
/// knows which field or value triggered the parse failure.
#[test]
fn test_parse_failed_display_contains_supplied_reason() {
    let err = RetryError::ParseFailed("missing field `max_retries`".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("max_retries"),
        "ParseFailed Display must embed the reason; got: {msg}"
    );
}

/// `ParseFailed` must not produce an empty display string.
#[test]
fn test_parse_failed_display_is_non_empty() {
    let err = RetryError::ParseFailed("x".to_string());
    assert!(!err.to_string().is_empty());
}

/// `ParseFailed` must be `Debug`-printable without panicking.
#[test]
fn test_parse_failed_is_debug_printable() {
    let _ = format!("{:?}", RetryError::ParseFailed("bad config".to_string()));
}

/// `ParseFailed` must be usable as a `std::error::Error` trait object.
#[test]
fn test_parse_failed_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(RetryError::ParseFailed("oops".to_string()));
    assert!(!err.to_string().is_empty());
}
