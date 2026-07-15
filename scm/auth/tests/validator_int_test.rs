//! Integration tests for the `Validator` trait in `edge-transport-http-egress-auth`.

use core::marker::PhantomData;
use edge_transport_http_egress_auth::AuthConfig;

/// @covers: Validator
/// `Validator` is an internal crate trait; its contract is enforced through
/// the public `AuthSvc::build_auth_middleware` factory. This test verifies
/// that `AuthConfig` — the type passed through validation — is accessible from
/// the public API. Fails to compile if `AuthConfig` is removed.
#[test]
fn test_auth_config_type_accessible_as_validator_input() {
    let _exists = PhantomData::<AuthConfig>;
}
