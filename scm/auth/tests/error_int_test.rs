//! Integration tests for `AuthError` — the public error enum.
//!
//! Each variant's `Display` string is tested for:
//! 1. Presence of the crate name so operators know the source.
//! 2. Presence of the payload so the message is actionable.
//! 3. The message differs per variant (no accidental conflation).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::AuthError;

// ---------------------------------------------------------------------------
// ParseFailed
// ---------------------------------------------------------------------------

#[test]
fn test_error_parse_failed_display_contains_crate_name() {
    let err = AuthError::ParseFailed("unexpected eof".into());
    let s = err.to_string();
    assert!(
        s.contains("edge_transport_http_egress_auth"),
        "ParseFailed Display must identify the crate: {s}"
    );
}

#[test]
fn test_error_parse_failed_display_contains_reason() {
    let reason = "missing field `kind`";
    let s = AuthError::ParseFailed(reason.into()).to_string();
    assert!(
        s.contains(reason),
        "ParseFailed Display must include the reason: {s}"
    );
}

#[test]
fn test_error_parse_failed_display_distinct_reasons_produce_distinct_messages() {
    let s1 = AuthError::ParseFailed("reason-one".into()).to_string();
    let s2 = AuthError::ParseFailed("reason-two".into()).to_string();
    assert_ne!(s1, s2, "different reasons must produce different messages");
}

// ---------------------------------------------------------------------------
// MissingEnvVar
// ---------------------------------------------------------------------------

#[test]
fn test_error_missing_env_var_display_contains_crate_name() {
    let err = AuthError::MissingEnvVar {
        name: "SOME_VAR".into(),
    };
    let s = err.to_string();
    assert!(
        s.contains("edge_transport_http_egress_auth"),
        "MissingEnvVar Display must identify the crate: {s}"
    );
}

#[test]
fn test_error_missing_env_var_display_contains_var_name() {
    let var_name = "MY_SPECIFIC_SECRET_ENV_VAR";
    let s = AuthError::MissingEnvVar {
        name: var_name.into(),
    }
    .to_string();
    assert!(
        s.contains(var_name),
        "MissingEnvVar Display must include the var name so operators know what to set: {s}"
    );
}

#[test]
fn test_error_missing_env_var_display_different_names_produce_different_messages() {
    let s1 = AuthError::MissingEnvVar {
        name: "VAR_A".into(),
    }
    .to_string();
    let s2 = AuthError::MissingEnvVar {
        name: "VAR_B".into(),
    }
    .to_string();
    assert_ne!(s1, s2);
}

// ---------------------------------------------------------------------------
// UnsupportedKind
// ---------------------------------------------------------------------------

#[test]
fn test_error_unsupported_kind_display_contains_crate_name() {
    let err = AuthError::UnsupportedKind {
        kind: "oauth2".into(),
    };
    let s = err.to_string();
    assert!(
        s.contains("edge_transport_http_egress_auth"),
        "UnsupportedKind Display must identify the crate: {s}"
    );
}

#[test]
fn test_error_unsupported_kind_display_contains_offending_kind() {
    let kind = "ntlm_unique_kind_marker";
    let s = AuthError::UnsupportedKind { kind: kind.into() }.to_string();
    assert!(
        s.contains(kind),
        "UnsupportedKind Display must include the offending kind: {s}"
    );
}

#[test]
fn test_error_unsupported_kind_display_mentions_valid_options() {
    // Operators need to know what values ARE valid so they can fix the
    // config. The Display message must mention at least one valid kind.
    let s = AuthError::UnsupportedKind { kind: "bad".into() }.to_string();
    assert!(
        s.contains("bearer") || s.contains("basic") || s.contains("none") || s.contains("header"),
        "UnsupportedKind Display must mention valid auth kinds: {s}"
    );
}

// ---------------------------------------------------------------------------
// InvalidHeaderValue
// ---------------------------------------------------------------------------

#[test]
fn test_error_invalid_header_value_display_contains_crate_name() {
    let err = AuthError::InvalidHeaderValue("CR in value".into());
    let s = err.to_string();
    assert!(
        s.contains("edge_transport_http_egress_auth"),
        "InvalidHeaderValue Display must identify the crate: {s}"
    );
}

#[test]
fn test_error_invalid_header_value_display_contains_reason() {
    let reason = "forbidden byte 0x0d";
    let s = AuthError::InvalidHeaderValue(reason.into()).to_string();
    assert!(
        s.contains(reason),
        "InvalidHeaderValue Display must include the reason: {s}"
    );
}

// ---------------------------------------------------------------------------
// InvalidHeaderName
// ---------------------------------------------------------------------------

#[test]
fn test_error_invalid_header_name_display_contains_crate_name() {
    let err = AuthError::InvalidHeaderName {
        name: "bad name".into(),
        reason: "spaces not allowed".into(),
    };
    let s = err.to_string();
    assert!(
        s.contains("edge_transport_http_egress_auth"),
        "InvalidHeaderName Display must identify the crate: {s}"
    );
}

#[test]
fn test_error_invalid_header_name_display_contains_offending_name() {
    let offender = "header with spaces";
    let s = AuthError::InvalidHeaderName {
        name: offender.into(),
        reason: "illegal chars".into(),
    }
    .to_string();
    assert!(
        s.contains(offender),
        "InvalidHeaderName Display must include the offending name: {s}"
    );
}

#[test]
fn test_error_invalid_header_name_display_contains_reason() {
    let reason = "spaces are not allowed in header names";
    let s = AuthError::InvalidHeaderName {
        name: "bad".into(),
        reason: reason.into(),
    }
    .to_string();
    assert!(
        s.contains(reason),
        "InvalidHeaderName Display must include the reason: {s}"
    );
}

// ---------------------------------------------------------------------------
// Variant distinctness — each variant produces a different message
// ---------------------------------------------------------------------------

#[test]
fn test_error_variants_produce_distinct_display_strings() {
    let msgs = [
        AuthError::ParseFailed("x".into()).to_string(),
        AuthError::MissingEnvVar { name: "x".into() }.to_string(),
        AuthError::UnsupportedKind { kind: "x".into() }.to_string(),
        AuthError::InvalidHeaderValue("x".into()).to_string(),
        AuthError::InvalidHeaderName {
            name: "x".into(),
            reason: "x".into(),
        }
        .to_string(),
    ];
    // Every message must be unique — they carry different prefixes.
    let unique: std::collections::HashSet<_> = msgs.iter().collect();
    assert_eq!(
        unique.len(),
        msgs.len(),
        "every Error variant must have a distinct Display format: {msgs:?}"
    );
}

// ---------------------------------------------------------------------------
// Debug impl — must not panic
// ---------------------------------------------------------------------------

#[test]
fn test_error_debug_impl_does_not_panic_for_any_variant() {
    let variants: Vec<AuthError> = vec![
        AuthError::ParseFailed("x".into()),
        AuthError::MissingEnvVar { name: "VAR".into() },
        AuthError::UnsupportedKind {
            kind: "oauth2".into(),
        },
        AuthError::InvalidHeaderValue("bad val".into()),
        AuthError::InvalidHeaderName {
            name: "bad-name".into(),
            reason: "reason".into(),
        },
    ];
    for err in variants {
        let _ = format!("{err:?}");
    }
}
