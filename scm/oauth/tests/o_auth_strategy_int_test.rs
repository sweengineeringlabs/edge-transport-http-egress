//! Integration tests for `OAuthStrategy` marker trait.
//!
//! Rule 120: `src/api/refresh/strategy/oauth/o_auth_strategy.rs` requires a
//! corresponding test file.
//!
//! `OAuthStrategy` is a marker trait. We verify the middleware stack that
//! implements it is wired correctly via the builder.

use std::sync::Arc;

use edge_transport_http_egress_oauth::{OAuthBuilder, OAuthBuilderOps, OAuthTokenSource, Result};
use futures::future::BoxFuture;

#[derive(Debug)]
struct DummySource;

impl OAuthTokenSource for DummySource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(async { Ok("strategy-token".into()) })
    }
}

/// @covers: OAuthStrategy (via builder flow)
/// The builder wires the refresh strategy (which implements `OAuthStrategy`)
/// into the middleware. Building with a valid source must succeed.
#[test]
fn oauth_trait_o_auth_strategy_builder_wires_strategy_int_test() {
    let src = Arc::new(DummySource);
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "builder with OAuthStrategy wired must succeed; got: {result:?}"
    );
}
