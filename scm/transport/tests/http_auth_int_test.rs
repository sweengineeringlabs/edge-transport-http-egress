//! Integration tests for `HttpAuth`.

use edge_transport_http_egress_transport::HttpAuth;

/// @covers: bearer
#[test]
fn test_http_auth_enum_bearer_creates_bearer_auth_with_token() {
    let auth = HttpAuth::bearer("tok_abc");
    assert!(matches!(auth, HttpAuth::Bearer { token } if token == "tok_abc"));
}

/// @covers: basic
#[test]
fn test_http_auth_enum_basic_creates_basic_auth_with_credentials() {
    let auth = HttpAuth::basic("user", "pass");
    assert!(matches!(auth, HttpAuth::Basic { username, .. } if username == "user"));
}

/// @covers: api_key
#[test]
fn test_http_auth_enum_api_key_creates_api_key_auth() {
    let auth = HttpAuth::api_key("X-Api-Key", "secret");
    assert!(matches!(auth, HttpAuth::ApiKey { header, .. } if header == "X-Api-Key"));
}
