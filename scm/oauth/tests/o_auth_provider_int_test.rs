//! Integration tests for `OAuthProvider`.
//!
//! Rule 120: `src/api/oauth/o_auth_provider.rs` requires a corresponding test file.

use edge_transport_http_egress_oauth::OAuthProvider;

/// @covers: OAuthProvider variants
/// Each variant must be distinct and comparable.
#[test]
fn oauth_struct_o_auth_provider_variants_are_distinct_int_test() {
    assert_ne!(
        OAuthProvider::Claude,
        OAuthProvider::Google,
        "Claude and Google must be distinct variants"
    );
    assert_ne!(
        OAuthProvider::Claude,
        OAuthProvider::OpenAi,
        "Claude and OpenAi must be distinct variants"
    );
    assert_ne!(
        OAuthProvider::Google,
        OAuthProvider::OpenAi,
        "Google and OpenAi must be distinct variants"
    );
}

/// @covers: OAuthProvider Debug
/// All variants must produce non-empty Debug output.
#[test]
fn oauth_struct_o_auth_provider_debug_non_empty_int_test() {
    for provider in [
        OAuthProvider::Claude,
        OAuthProvider::Google,
        OAuthProvider::OpenAi,
    ] {
        let dbg = format!("{provider:?}");
        assert!(!dbg.is_empty(), "OAuthProvider Debug must be non-empty");
    }
}

/// @covers: OAuthProvider Clone
/// Cloning a variant must produce an equal copy.
#[test]
fn oauth_struct_o_auth_provider_clone_equals_original_int_test() {
    let original = OAuthProvider::Claude;
    let cloned = original.clone();
    assert_eq!(original, cloned, "cloned OAuthProvider must equal original");
}
