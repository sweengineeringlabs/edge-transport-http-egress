//! Integration tests for `edge_transport_http_egress_cassette::CassetteError`.
//!
//! Covers: `CassetteError::ParseFailed`, `CassetteError::NotImplemented` — Display messages
//! must be actionable: name the crate, embed the payload, be non-empty.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::CassetteError;

// ---------------------------------------------------------------------------
// CassetteError::ParseFailed
// ---------------------------------------------------------------------------

/// `ParseFailed` display must name the crate so an operator can trace the
/// error back to `edge_transport_http_egress_cassette` without reading source code.
#[test]
fn test_parse_failed_display_names_the_crate() {
    let err = CassetteError::ParseFailed("unexpected key 'mode2'".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("edge_transport_http_egress_cassette"),
        "ParseFailed Display must name the crate; got: {msg}"
    );
}

/// `ParseFailed` display must echo the wrapped reason so the operator
/// knows which field or value triggered the failure.
#[test]
fn test_parse_failed_display_contains_supplied_reason() {
    let err = CassetteError::ParseFailed("missing field `cassette_dir`".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("cassette_dir"),
        "ParseFailed Display must contain the reason; got: {msg}"
    );
}

/// `ParseFailed` must be `Debug`-printable without panicking.
#[test]
fn test_parse_failed_is_debug_printable() {
    let err = CassetteError::ParseFailed("bad config".to_string());
    let _ = format!("{err:?}");
}

/// `ParseFailed` must be displayable as a `std::error::Error` trait object,
/// confirming the `thiserror` derive wires up the standard trait correctly.
#[test]
fn test_parse_failed_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(CassetteError::ParseFailed("oops".to_string()));
    assert!(!err.to_string().is_empty());
}
