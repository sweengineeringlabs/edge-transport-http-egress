//! Integration tests for `api/error.rs` — the public `BreakerError` enum.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::BreakerError;

// ---------------------------------------------------------------------------
// BreakerError::ParseFailed
// ---------------------------------------------------------------------------

/// `Display` must name the crate.
#[test]
fn test_error_parse_failed_display_names_crate() {
    let err = BreakerError::ParseFailed("bad field".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("edge_transport_http_egress_breaker"),
        "ParseFailed display must name the crate; got: {msg}"
    );
}

/// `Display` must echo the supplied reason.
#[test]
fn test_error_parse_failed_display_echoes_reason() {
    let reason = "missing field `failure_threshold`";
    let err = BreakerError::ParseFailed(reason.to_string());
    assert!(
        err.to_string().contains(reason),
        "ParseFailed display must contain the reason; got: {}",
        err
    );
}

/// Empty reason still produces a non-empty display.
#[test]
fn test_error_parse_failed_with_empty_reason_display_is_non_empty() {
    let err = BreakerError::ParseFailed(String::new());
    assert!(
        !err.to_string().is_empty(),
        "ParseFailed display must not be empty even with empty reason"
    );
}

// ---------------------------------------------------------------------------
// Debug
// ---------------------------------------------------------------------------

/// `Debug` output must name the variant, not just wrap the message opaquely.
#[test]
fn test_error_variants_implement_debug() {
    let dbg = format!("{:?}", BreakerError::ParseFailed("p".to_string()));
    assert!(
        dbg.contains("ParseFailed"),
        "Debug output must name the ParseFailed variant: {dbg}"
    );
}
