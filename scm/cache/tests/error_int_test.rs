//! Integration tests for `api/error.rs` — the public `Error` enum.
//!
//! Verifies that both variants are publicly constructable, that `Display` output
//! is actionable (names the crate + includes the reason), and that `Debug` is
//! available.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::CacheError;

// ---------------------------------------------------------------------------
// CacheError::ParseFailed
// ---------------------------------------------------------------------------

/// `ParseFailed` must be publicly constructable so callers can pattern-match
/// it.  If the variant or its inner `String` field becomes private, this test
/// fails to compile.
#[test]
fn test_error_parse_failed_is_publicly_constructable() {
    let err = CacheError::ParseFailed("any reason".to_string());
    // Prove the variant genuinely carries its payload through Display, not
    // just that the constructor exists.
    assert!(
        err.to_string().contains("any reason"),
        "ParseFailed must retain its reason payload; got: {err}"
    );
}

/// The `Display` output for `ParseFailed` must contain the crate name so an
/// operator reading a log line can immediately identify the source.
#[test]
fn test_error_parse_failed_display_names_crate() {
    let err = CacheError::ParseFailed("bad field".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("edge_transport_http_egress_cache"),
        "ParseFailed display must name the crate; got: {msg}"
    );
}

/// The `Display` output must echo the supplied reason verbatim so the operator
/// knows exactly which TOML field or value caused the failure.
#[test]
fn test_error_parse_failed_display_echoes_reason() {
    let reason = "missing field `max_entries`";
    let err = CacheError::ParseFailed(reason.to_string());
    let msg = err.to_string();
    assert!(
        msg.contains(reason),
        "ParseFailed display must contain the reason string; got: {msg}"
    );
}

/// An empty reason string must still produce a non-empty `Display` message —
/// the crate name alone satisfies the "actionable" requirement.
#[test]
fn test_error_parse_failed_with_empty_reason_display_is_non_empty() {
    let err = CacheError::ParseFailed(String::new());
    assert!(
        !err.to_string().is_empty(),
        "ParseFailed display must not be empty even with empty reason"
    );
}

// ---------------------------------------------------------------------------
// Debug
// ---------------------------------------------------------------------------

/// `ParseFailed` must implement `Debug` so it can appear in `{:?}` log
/// output without a manual trait impl.
#[test]
fn test_error_variants_implement_debug() {
    let parse_err = CacheError::ParseFailed("x".to_string());
    let dbg = format!("{parse_err:?}");
    assert!(
        dbg.contains("ParseFailed") && dbg.contains('x'),
        "Debug must name the variant and include its payload; got: {dbg}"
    );
}
