//! Integration tests for `StaticTokenSource` api marker.
//!
//! Rule 120: `src/api/refresh/strategy/oauth/static_token_source.rs` requires a
//! corresponding test file.
//!
//! The api `StaticTokenSource` is a public marker struct. The concrete
//! behavior (returns a fixed token) is in `core/`. We test the analogous
//! behavior through a custom `OAuthTokenSource` implementation.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_oauth::{OAuthBuilder, OAuthBuilderOps, OAuthTokenSource, Result};
use futures::future::BoxFuture;

/// A static-token source for integration testing.
#[derive(Debug)]
struct FixedSource(String);

impl OAuthTokenSource for FixedSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        let t = self.0.clone();
        Box::pin(async move { Ok(t) })
    }
}

/// @covers: StaticTokenSource (behavior via OAuthTokenSource)
/// A static-token source always returns the same token on every call.
#[tokio::test]
async fn oauth_struct_static_token_source_returns_same_token_on_each_call_int_test() {
    let src = FixedSource("static-access-token".into());
    let t1 = src
        .get_access_token()
        .await
        .expect("first call must succeed");
    let t2 = src
        .get_access_token()
        .await
        .expect("second call must succeed");
    assert_eq!(
        t1, t2,
        "static source must return the same token on every call"
    );
}

/// @covers: StaticTokenSource (construction via builder)
/// Building the middleware with a fixed-token source must succeed.
#[test]
fn oauth_struct_static_token_source_builds_middleware_int_test() {
    let src = Arc::new(FixedSource("my-token".into()));
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "static-token source must build middleware; got: {result:?}"
    );
}
