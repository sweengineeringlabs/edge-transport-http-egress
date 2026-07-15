//! Integration tests for `OAuthError`.
//!
//! Rule 120: `src/api/error/o_auth_error.rs` requires a corresponding test file.

use edge_transport_http_egress_oauth::OAuthError;

/// @covers: OAuthError variants
/// Each variant must display a message that names the crate and the error kind.
#[test]
fn oauth_struct_o_auth_error_credentials_not_found_display_int_test() {
    let err = OAuthError::CredentialsNotFound("test-key".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("credentials not found"),
        "CredentialsNotFound display must describe the error; got: {msg}"
    );
}

/// @covers: OAuthError::RefreshFailed
#[test]
fn oauth_struct_o_auth_error_refresh_failed_display_int_test() {
    let err = OAuthError::RefreshFailed("network timeout".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("token refresh failed"),
        "RefreshFailed display must describe the error; got: {msg}"
    );
}

/// @covers: OAuthError::Http
#[test]
fn oauth_struct_o_auth_error_http_display_int_test() {
    let err = OAuthError::Http("connection refused".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("http error"),
        "Http variant display must contain 'http error'; got: {msg}"
    );
}

/// @covers: OAuthError::Configuration
#[test]
fn oauth_struct_o_auth_error_configuration_display_int_test() {
    let err = OAuthError::Configuration("missing field".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("configuration error"),
        "Configuration display must describe the error; got: {msg}"
    );
}

/// @covers: OAuthError Debug
/// Every variant must produce non-empty Debug output.
#[test]
fn oauth_struct_o_auth_error_debug_non_empty_int_test() {
    for err in [
        OAuthError::CredentialsNotFound("x".into()),
        OAuthError::RefreshFailed("x".into()),
        OAuthError::Http("x".into()),
        OAuthError::Configuration("x".into()),
    ] {
        let dbg = format!("{err:?}");
        assert!(!dbg.is_empty(), "OAuthError Debug must be non-empty");
    }
}
