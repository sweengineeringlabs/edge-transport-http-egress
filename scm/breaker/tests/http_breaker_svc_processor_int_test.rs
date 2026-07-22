//! Integration tests for `HttpBreakerSvcProcessor` — the crate's SAF facade type.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerConfig, DescribeRequest, HttpBreakerSvcProcessor, Processor,
};

/// @covers: HttpBreakerSvcProcessor
#[test]
fn test_http_breaker_svc_is_constructible_as_a_unit_value_happy() {
    let svc = HttpBreakerSvcProcessor;
    // HttpBreakerSvcProcessor doubles as this crate's Processor implementor.
    let resp = svc.describe(DescribeRequest).expect("infallible");
    assert_eq!(resp.value, "http-breaker");
}

/// @covers: HttpBreakerSvcProcessor
#[test]
fn test_http_breaker_svc_static_methods_are_independent_of_instance_edge() {
    // create_config_builder / build_breaker_layer are associated functions,
    // not instance methods — reachable without ever constructing a value.
    let builder = HttpBreakerSvcProcessor::create_config_builder();
    assert!(!builder.name().is_empty());
    let layer = HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default());
    assert!(layer.is_ok());
}
