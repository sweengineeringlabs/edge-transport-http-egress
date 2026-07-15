//! Integration tests for `OAuthBuilderOps` trait.
//!
//! Rule 120: `src/api/oauth/o_auth_builder_ops.rs` requires a corresponding test file.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_oauth::{OAuthBuilder, OAuthBuilderOps, OAuthTokenSource, Result};
use futures::future::BoxFuture;

#[derive(Debug)]
struct DummySource;

impl OAuthTokenSource for DummySource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(async { Ok("token".into()) })
    }
}

/// @covers: OAuthBuilderOps::with_token_source
/// The `with_token_source` method on an `OAuthBuilder` must be callable.
#[test]
fn oauth_trait_o_auth_builder_ops_with_token_source_is_callable_int_test() {
    let src: Arc<dyn OAuthTokenSource> = Arc::new(DummySource);
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "OAuthBuilderOps::with_token_source + build must succeed; got: {result:?}"
    );
}

/// @covers: OAuthBuilderOps::build
/// `build()` without a source must return a `Configuration` error.
#[test]
fn oauth_trait_o_auth_builder_ops_build_without_source_returns_configuration_error_int_test() {
    use edge_transport_http_egress_oauth::OAuthError;
    let err = OAuthBuilder::new().build().unwrap_err();
    assert!(
        matches!(err, OAuthError::Configuration(_)),
        "missing source must yield OAuthError::Configuration; got: {err:?}"
    );
}
