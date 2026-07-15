//! Integration tests for the `Validator` trait in `edge-transport-http-egress-oauth`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_oauth::{
    OAuthBuilder, OAuthBuilderOps, OAuthError, OAuthMiddleware, OAuthTokenSource, Result,
};
use futures::future::BoxFuture;

#[derive(Debug)]
struct ValidSource;

impl OAuthTokenSource for ValidSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(async { Ok("validated-token".into()) })
    }
}

/// @covers: Validator (via OAuthRefreshStrategy which implements Validator internally)
/// `OAuthMiddleware` is constructed from `OAuthRefreshStrategy`, which must satisfy
/// the `Validator` contract. A successful build proves the validator is wired and
/// accepts a source with valid credentials at construction time.
#[test]
fn oauth_trait_validator_is_wired_into_middleware_int_test() {
    let src: Arc<dyn OAuthTokenSource> = Arc::new(ValidSource);
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "Validator-wired OAuthMiddleware must build successfully; got: {result:?}"
    );
    // The type must be OAuthMiddleware — verifies the Validator constraint is satisfied.
    let _: OAuthMiddleware = result.unwrap();
}

/// @covers: Validator (rejection path — build without source)
/// Without a token source the builder must return a Configuration error,
/// confirming the validator's credential-check path is reachable.
#[test]
fn oauth_trait_validator_build_without_source_yields_configuration_error_int_test() {
    let err = OAuthBuilder::new().build().unwrap_err();
    assert!(
        matches!(err, OAuthError::Configuration(_)),
        "missing source must yield OAuthError::Configuration; got: {err:?}"
    );
}
