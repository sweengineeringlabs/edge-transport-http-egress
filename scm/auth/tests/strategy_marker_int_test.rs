//! @covers: api::strategy::traits::AuthStrategy
//!
//! `Strategy` (the marker trait in api/strategy/traits/strategy.rs) is pub(crate)
//! and cannot be imported in integration tests. These tests cover the public equivalent —
//! `AuthStrategy` — which is the functional strategy contract exported from the crate root.

use edge_transport_http_egress_auth::AuthStrategy;

/// @covers: api::strategy::traits::AuthStrategy — exported from crate root.
///
/// Fails to compile if AuthStrategy is not re-exported from saf/.
#[test]
fn test_auth_strategy_trait_is_accessible() {
    let _: std::marker::PhantomData<Box<dyn AuthStrategy>>;
}

/// @covers: api::strategy::traits::AuthStrategy — object-safe.
///
/// Fails to compile if a non-object-safe method is added to AuthStrategy.
#[test]
fn test_auth_strategy_is_object_safe() {
    fn _accept(_: &dyn AuthStrategy) {}
}

/// @covers: api::strategy::traits::AuthStrategy — Send + Sync supertrait bounds.
///
/// Fails to compile if Send + Sync supertraits are removed from AuthStrategy.
#[test]
fn test_auth_strategy_is_send_sync() {
    fn _assert_send_sync<T: Send + Sync + ?Sized>() {}
    _assert_send_sync::<dyn AuthStrategy>();
}
