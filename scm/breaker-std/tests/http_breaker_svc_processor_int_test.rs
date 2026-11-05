//! Integration tests for the processor and construction paths of `StdBreakerFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::ConfigBuilder as _;
use edge_transport_http_egress_breaker_api::{BreakerConfig, BreakerFactory, DescribeRequest};
use edge_transport_http_egress_breaker_std::StdBreakerFactory;

/// @covers: create_processor
#[test]
fn test_create_processor_describes_the_http_breaker_happy() {
    let svc = StdBreakerFactory.create_processor();
    let resp = svc.describe(DescribeRequest).expect("infallible");
    assert_eq!(resp.value, "http-breaker");
}

/// @covers: create_config_builder
/// @covers: build_breaker_layer
#[test]
fn test_factory_construction_methods_need_no_state_edge() {
    // The factory is a stateless unit value: both construction paths work on a fresh one.
    let builder = StdBreakerFactory.create_config_builder();
    assert!(!builder.name().is_empty());
    let layer = StdBreakerFactory.build_breaker_layer(BreakerConfig::default());
    assert!(layer.is_ok());
}
