//! Integration tests for `OAuthConfig`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_oauth::{OAuthConfig, OAuthProvider};

/// @covers: OAuthConfig::from_config
#[test]
fn oauth_struct_o_auth_config_from_config_parses_claude_int_test() {
    let cfg = OAuthConfig::from_config(r#"provider = "claude""#).expect("must parse");
    assert_eq!(cfg.provider, OAuthProvider::Claude);
    assert!(cfg.credentials_path.is_none());
}

/// @covers: OAuthConfig::from_config
#[test]
fn oauth_struct_o_auth_config_from_config_parses_google_with_path_int_test() {
    let cfg = OAuthConfig::from_config(
        r#"provider = "google"
credentials_path = "/custom/creds.json""#,
    )
    .expect("must parse");
    assert_eq!(cfg.provider, OAuthProvider::Google);
    assert_eq!(cfg.credentials_path.as_deref(), Some("/custom/creds.json"));
}

/// @covers: OAuthConfig::from_config
#[test]
fn oauth_struct_o_auth_config_from_config_parses_open_ai_int_test() {
    let cfg = OAuthConfig::from_config(r#"provider = "open_ai""#).expect("must parse");
    assert_eq!(cfg.provider, OAuthProvider::OpenAi);
}

/// @covers: OAuthConfig::default
#[test]
fn oauth_struct_o_auth_config_default_is_claude_int_test() {
    let cfg = OAuthConfig::default();
    assert_eq!(cfg.provider, OAuthProvider::Claude);
    assert!(cfg.credentials_path.is_none());
}
