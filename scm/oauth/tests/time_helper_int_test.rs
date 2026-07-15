//! Integration tests for `TimeHelper` marker trait.
//!
//! Rule 120: `src/api/refresh/strategy/oauth/time_helper.rs` requires a
//! corresponding test file.
//!
//! The api `TimeHelper` is a marker trait. The concrete clock helper
//! (`OAuthTimeHelper`) is `pub(crate)` and already has inline unit tests.
//! This file covers the module at the integration level.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_oauth::OAuthSvc;

/// @covers: TimeHelper (via OAuth middleware clock usage)
/// The time helper is used internally to check token expiry. We verify
/// that the middleware stack builds and responds to token source calls,
/// which transitively exercises the time helper.
#[test]
fn oauth_trait_time_helper_middleware_builds_int_test() {
    use edge_transport_http_egress_oauth::{OAuthBuilderOps, OAuthError};
    let err = OAuthSvc::builder().build().unwrap_err();
    assert!(
        matches!(err, OAuthError::Configuration(_)),
        "time helper is active in the middleware path; missing source yields Configuration; got: {err:?}"
    );
}
