//! Integration tests for `OAuthCredentials`.
//!
//! Rule 120: `src/api/oauth/o_auth_credentials.rs` requires a corresponding test file.

use edge_transport_http_egress_oauth::OAuthCredentials;

/// @covers: OAuthCredentials construction
/// All fields must be settable and readable.
#[test]
fn oauth_struct_o_auth_credentials_fields_accessible_int_test() {
    let at = "access-xyz".to_string();
    let rt = "refresh-abc".to_string();
    let creds = OAuthCredentials {
        access_token: at,
        refresh_token: rt,
        expires_at_ms: 1_700_000_000_000u64,
        scopes: vec!["read".to_string(), "write".to_string()],
    };
    assert_eq!(creds.access_token, "access-xyz");
    assert_eq!(creds.refresh_token, "refresh-abc");
    assert_eq!(creds.expires_at_ms, 1_700_000_000_000u64);
    assert_eq!(creds.scopes, vec!["read", "write"]);
}

/// @covers: OAuthCredentials Debug
/// Debug output must not be empty.
#[test]
fn oauth_struct_o_auth_credentials_debug_non_empty_int_test() {
    let creds = OAuthCredentials {
        access_token: "tok".to_string(),
        refresh_token: "ref".to_string(),
        expires_at_ms: 0,
        scopes: vec![],
    };
    let dbg = format!("{creds:?}");
    assert!(!dbg.is_empty(), "OAuthCredentials Debug must be non-empty");
}

/// @covers: OAuthCredentials Clone
/// Cloning must produce an independent copy.
#[test]
fn oauth_struct_o_auth_credentials_clone_is_independent_int_test() {
    let original = OAuthCredentials {
        access_token: "tok".to_string(),
        refresh_token: "ref".to_string(),
        expires_at_ms: 1_000,
        scopes: vec!["scope".to_string()],
    };
    let mut cloned = original.clone();
    let new_val = "different".to_string();
    cloned.access_token = new_val;
    assert_eq!(
        original.access_token, "tok",
        "original must be unaffected after mutating clone"
    );
}
