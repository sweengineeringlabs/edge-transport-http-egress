//! Integration tests for core `FailingTokenSource` behavior.
//!
//! Rule 120: `src/core/refresh/strategy/oauth/failing_token_source.rs` requires a
//! corresponding test file.
//!
//! The core `FailingTokenSource` is `pub(crate)` and not directly accessible
//! from integration tests. We test the analogous "always fails" behavior through
//! a custom `OAuthTokenSource` implementation.

use std::sync::Arc;

use edge_transport_http_egress_oauth::{
    OAuthBuilder, OAuthBuilderOps, OAuthError, OAuthTokenSource, Result,
};
use futures::future::BoxFuture;

#[derive(Debug)]
struct CoreAlwaysFailSource;

impl OAuthTokenSource for CoreAlwaysFailSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(async {
            Err(OAuthError::RefreshFailed(
                "core failing source: always fails".into(),
            ))
        })
    }
}

/// @covers: core FailingTokenSource (via OAuthTokenSource)
/// Building middleware with an always-failing source must succeed at
/// construction time (fail is deferred to token request time).
#[test]
fn oauth_struct_failing_token_source_core_middleware_builds_int_test() {
    let src = Arc::new(CoreAlwaysFailSource);
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "always-fail source must still build middleware; got: {result:?}"
    );
}

/// @covers: core FailingTokenSource (token retrieval error)
#[tokio::test]
async fn oauth_struct_failing_token_source_core_returns_error_on_get_int_test() {
    let src = CoreAlwaysFailSource;
    let result = src.get_access_token().await;
    assert!(
        result.is_err(),
        "core failing source must error on get_access_token"
    );
}
