//! Integration tests for `api/traits/mod.rs`.
//!
//! `traits/mod.rs` holds `Processor`, `BreakerMetrics`, `CircuitBreakerNode`, `HostBreaker`,
//! and `Validator` contracts.
//! From outside the crate, the observable effect is that `BreakerLayerBreakerMetrics` must satisfy
//! `Send + Sync` (the supertraits of `Processor`) so it can be stored behind a
//! trait object.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::BreakerLayerBreakerMetrics;

/// @covers: BreakerLayerBreakerMetrics
#[test]
fn test_breaker_layer_coercible_to_boxed_send_sync() {
    use edge_transport_http_egress_breaker::{BreakerConfig, HttpBreakerSvcProcessor};
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 5,
        reset_after_successes: 2,
        failure_statuses: vec![500],
    };
    let layer: BreakerLayerBreakerMetrics =
        HttpBreakerSvcProcessor::build_breaker_layer(cfg).expect("build");
    let _boxed: Box<dyn Send + Sync> = Box::new(layer);
}
