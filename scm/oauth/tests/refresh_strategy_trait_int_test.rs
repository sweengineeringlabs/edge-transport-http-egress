//! @covers: api::refresh::traits::OAuthTokenSource + OAuthError
//!
//! `RefreshStrategy` (the marker trait in api/refresh/traits/refresh_strategy.rs) is pub(crate)
//! and cannot be imported in integration tests. These tests cover the public equivalent —
//! `OAuthTokenSource` — which is the functional strategy contract exported from the crate root.

use edge_transport_http_egress_oauth::{OAuthError, OAuthTokenSource};

/// @covers: api::refresh::traits::OAuthTokenSource — exported from crate root.
///
/// Fails to compile if OAuthTokenSource is not re-exported from saf/.
#[test]
fn test_oauth_token_source_trait_is_accessible() {
    let _: std::marker::PhantomData<Box<dyn OAuthTokenSource>>;
}

/// @covers: api::refresh::traits::OAuthTokenSource — object-safe.
///
/// Fails to compile if a non-object-safe method is added to OAuthTokenSource.
#[test]
fn test_oauth_token_source_is_object_safe() {
    fn _accept(_: &dyn OAuthTokenSource) {}
}

/// @covers: api::refresh::traits::OAuthTokenSource — Send + Sync supertrait bounds.
///
/// Fails to compile if Send + Sync supertraits are removed from OAuthTokenSource.
#[test]
fn test_oauth_token_source_is_send_sync() {
    fn _assert_send_sync<T: Send + Sync + ?Sized>() {}
    _assert_send_sync::<dyn OAuthTokenSource>();
}

/// @covers: error path — OAuthError is accessible and constructable.
///
/// Fails if OAuthError is not exported or its variants are removed.
#[test]
fn test_oauth_error_credentials_not_found_variant_is_accessible() {
    let err = OAuthError::CredentialsNotFound("missing".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("credentials not found"),
        "error message should describe the problem: {msg}"
    );
}
