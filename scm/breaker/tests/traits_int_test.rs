//! Integration tests for `api/traits/mod.rs`.
//!
//! `traits/mod.rs` holds `Processor`, `Validator`, and `CircuitBreakerNode` contracts.
//! From outside the crate, the observable effect is that `BreakerLayer` must satisfy
//! `Send + Sync` (the supertraits of `Processor`) so it can be stored behind a
//! trait object.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::BreakerLayer;

/// @covers: BreakerLayer
#[test]
fn test_breaker_layer_satisfies_send_required_by_http_breaker_trait() {
    fn assert_send<T: Send>() {}
    assert_send::<BreakerLayer>();
}

/// @covers: BreakerLayer
#[test]
fn test_breaker_layer_satisfies_sync_required_by_http_breaker_trait() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<BreakerLayer>();
}

/// @covers: BreakerLayer
#[test]
fn test_breaker_layer_coercible_to_boxed_send_sync() {
    use edge_transport_http_egress_breaker::{BreakerConfig, HttpBreakerSvc};
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 5,
        reset_after_successes: 2,
        failure_statuses: vec![500],
    };
    let layer: BreakerLayer = HttpBreakerSvc::build_breaker_layer(cfg).expect("build");
    let _boxed: Box<dyn Send + Sync> = Box::new(layer);
}
