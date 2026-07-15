//! Integration tests for `OAuthTokenSource` trait.
//!
//! Rule 120: `src/api/oauth/o_auth_token_source.rs` requires a corresponding test file.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_oauth::{OAuthTokenSource, Result};
use futures::future::BoxFuture;

/// A minimal `OAuthTokenSource` implementation for testing object safety and
/// `get_access_token` return behavior.
#[derive(Debug)]
struct FixedTokenSource(String);

impl OAuthTokenSource for FixedTokenSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        let token = self.0.clone();
        Box::pin(async move { Ok(token) })
    }
}

/// @covers: OAuthTokenSource object safety
/// The trait must be object-safe (can be used as `dyn OAuthTokenSource`).
#[test]
fn oauth_trait_o_auth_token_source_is_object_safe_int_test() {
    let _: Arc<dyn OAuthTokenSource> = Arc::new(FixedTokenSource("token".into()));
}

/// @covers: OAuthTokenSource::get_access_token
/// `get_access_token` must return the expected token when called.
#[tokio::test]
async fn oauth_trait_o_auth_token_source_get_access_token_returns_token_int_test() {
    let src = FixedTokenSource("my-access-token".into());
    let result = src.get_access_token().await;
    assert!(result.is_ok(), "get_access_token must succeed");
    assert_eq!(
        result.unwrap(),
        "my-access-token",
        "get_access_token must return the configured token"
    );
}
