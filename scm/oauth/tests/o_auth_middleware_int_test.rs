//! Integration tests for `OAuthMiddleware`.
//!
//! Rule 120: `src/api/oauth/o_auth_middleware.rs` requires a corresponding test file.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_oauth::{
    OAuthBuilder, OAuthBuilderOps, OAuthMiddleware, OAuthTokenSource, Result,
};
use futures::future::BoxFuture;

#[derive(Debug)]
struct DummySource;

impl OAuthTokenSource for DummySource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(async { Ok("middleware-token".into()) })
    }
}

/// @covers: OAuthMiddleware construction via builder
/// Building the middleware from a source must produce an `OAuthMiddleware`.
#[test]
fn oauth_struct_o_auth_middleware_builds_from_source_int_test() {
    let src = Arc::new(DummySource);
    let middleware: OAuthMiddleware = OAuthBuilder::new()
        .with_token_source(src)
        .build()
        .expect("build must succeed");
    let dbg = format!("{middleware:?}");
    assert!(
        dbg.contains("OAuthMiddleware"),
        "OAuthMiddleware Debug must name the struct; got: {dbg}"
    );
}

/// @covers: OAuthMiddleware is reqwest_middleware::Middleware
/// The middleware must satisfy the `Middleware` bound (compile-time check).
#[test]
fn oauth_struct_o_auth_middleware_satisfies_middleware_bound_int_test() {
    fn assert_middleware<T: reqwest_middleware::Middleware>() {}
    assert_middleware::<OAuthMiddleware>();
}
