//! Integration tests for `OAuthBuilder`.
//!
//! Rule 120: `src/api/oauth/o_auth_builder.rs` requires a corresponding test file.

use std::sync::Arc;

use edge_transport_http_egress_oauth::{OAuthBuilder, OAuthBuilderOps, OAuthTokenSource, Result};
use futures::future::BoxFuture;

#[derive(Debug)]
struct DummySource;

impl OAuthTokenSource for DummySource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(async { Ok("dummy-token".into()) })
    }
}

/// @covers: OAuthBuilder::new
/// The builder is constructible via `new()`.
#[test]
fn oauth_struct_o_auth_builder_new_returns_empty_builder_int_test() {
    let _builder = OAuthBuilder::new();
}

/// @covers: OAuthBuilder::build
/// Building without a token source must fail.
#[test]
fn oauth_struct_o_auth_builder_build_without_source_fails_int_test() {
    let result = OAuthBuilder::new().build();
    assert!(
        result.is_err(),
        "build without token source must return an error"
    );
}

/// @covers: OAuthBuilder::with_token_source + build
/// Building with a token source must succeed.
#[test]
fn oauth_struct_o_auth_builder_build_with_source_succeeds_int_test() {
    let src = Arc::new(DummySource);
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "build with a token source must succeed; got: {result:?}"
    );
}
