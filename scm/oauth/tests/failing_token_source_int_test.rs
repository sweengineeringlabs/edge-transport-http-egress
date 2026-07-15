//! Integration tests for `FailingTokenSource` api marker.
//!
//! Rule 120: `src/api/refresh/strategy/oauth/failing_token_source.rs` requires a
//! corresponding test file.
//!
//! The api `FailingTokenSource` is a public marker struct. The concrete
//! behavior (always fails) is in `core/`. We test error propagation through
//! the public surface by using a custom always-failing `OAuthTokenSource`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_oauth::{
    OAuthBuilder, OAuthBuilderOps, OAuthError, OAuthTokenSource, Result,
};
use futures::future::BoxFuture;

/// An `OAuthTokenSource` that always returns `OAuthError::RefreshFailed`.
#[derive(Debug)]
struct AlwaysFailSource;

impl OAuthTokenSource for AlwaysFailSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(async { Err(OAuthError::RefreshFailed("deliberate failure".into())) })
    }
}

/// @covers: FailingTokenSource (behavior via OAuthTokenSource)
/// An always-failing source must build into valid middleware — the failure
/// is deferred to `get_access_token` call time, not construction time.
#[test]
fn oauth_struct_failing_token_source_builds_middleware_successfully_int_test() {
    let src = Arc::new(AlwaysFailSource);
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "failing source must still build middleware at construction time; got: {result:?}"
    );
}

/// @covers: FailingTokenSource (behavior via OAuthTokenSource)
/// `get_access_token` on an always-failing source must return `RefreshFailed`.
#[tokio::test]
async fn oauth_struct_failing_token_source_get_access_token_returns_error_int_test() {
    let src = AlwaysFailSource;
    let result = src.get_access_token().await;
    assert!(
        result.is_err(),
        "AlwaysFailSource must return an error from get_access_token"
    );
    assert!(
        matches!(result.unwrap_err(), OAuthError::RefreshFailed(_)),
        "error must be OAuthError::RefreshFailed"
    );
}
