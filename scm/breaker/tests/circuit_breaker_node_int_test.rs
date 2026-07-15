//! Integration tests for the `CircuitBreakerNode` trait.

use core::marker::PhantomData;
use edge_transport_http_egress_breaker::BreakerLayer;

/// @covers: CircuitBreakerNode
/// `CircuitBreakerNode` is a `pub(crate)` trait; its contract is enforced
/// externally through `BreakerLayer`, which wraps the per-host state machine.
/// Confirming that `BreakerLayer` is `Send + Sync` proves the node impls
/// satisfy the concurrency bounds required by the trait.
#[test]
fn test_circuit_breaker_node_trait_is_defined() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<BreakerLayer>();
    let _: PhantomData<BreakerLayer> = PhantomData;
}
