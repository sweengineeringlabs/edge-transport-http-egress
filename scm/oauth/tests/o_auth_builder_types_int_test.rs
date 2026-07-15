//! Integration tests for `OAuthBuilder` api types.
//!
//! Rule 120: `src/api/types/o_auth_builder.rs` requires a corresponding test file.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_oauth::{OAuthBuilder, OAuthBuilderOps, OAuthTokenSource, Result};
use futures::future::BoxFuture;

#[derive(Debug)]
struct DummySource;

impl OAuthTokenSource for DummySource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(async { Ok("types-token".into()) })
    }
}

/// @covers: OAuthBuilder (api/types/o_auth_builder.rs)
/// The `OAuthBuilder` type in `api/types` must be the same type exported by SAF.
/// Building with a valid source must succeed.
#[test]
fn oauth_struct_o_auth_builder_types_builds_middleware_int_test() {
    let src: Arc<dyn OAuthTokenSource> = Arc::new(DummySource);
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "OAuthBuilder from api/types must build middleware; got: {result:?}"
    );
}

/// @covers: OAuthBuilder::new (default)
/// `OAuthBuilder::new()` and `Default::default()` must produce equivalent builders.
#[test]
fn oauth_struct_o_auth_builder_types_new_and_default_are_equivalent_int_test() {
    // Both must fail to build with no source — proving neither has a pre-wired source.
    let err_new = OAuthBuilder::new().build().unwrap_err();
    let err_default = OAuthBuilder::default().build().unwrap_err();
    assert_eq!(
        err_new.to_string(),
        err_default.to_string(),
        "new() and default() must produce identical errors when built without source"
    );
}
