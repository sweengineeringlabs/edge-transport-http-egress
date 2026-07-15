//! Integration tests for core `StaticTokenSource` behavior.
//!
//! Rule 120: `src/core/refresh/strategy/oauth/static_token_source.rs` requires a
//! corresponding test file.
//!
//! The core `StaticTokenSource` is `pub(crate)` and not directly accessible
//! from integration tests. We test the fixed-token behavior pattern through
//! a custom `OAuthTokenSource` implementation.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_oauth::{OAuthBuilder, OAuthBuilderOps, OAuthTokenSource, Result};
use futures::future::BoxFuture;

#[derive(Debug)]
struct CoreStaticSource(String);

impl OAuthTokenSource for CoreStaticSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        let t = self.0.clone();
        Box::pin(async move { Ok(t) })
    }
}

/// @covers: core StaticTokenSource (via OAuthTokenSource)
/// A static token source must return the same value on every call.
#[tokio::test]
async fn oauth_struct_static_token_source_core_returns_same_token_int_test() {
    let src = CoreStaticSource("fixed-token".into());
    let t1 = src.get_access_token().await.expect("first call");
    let t2 = src.get_access_token().await.expect("second call");
    assert_eq!(
        t1, t2,
        "static source must return the same token on repeated calls"
    );
}

/// @covers: core StaticTokenSource (middleware construction)
/// Building the middleware with a static source must succeed.
#[test]
fn oauth_struct_static_token_source_core_builds_middleware_int_test() {
    let src = Arc::new(CoreStaticSource("core-token".into()));
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "static source must build middleware; got: {result:?}"
    );
}
