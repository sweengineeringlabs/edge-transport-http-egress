//! Integration tests for `DescribeRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker_api::{BreakerFactory, DescribeRequest};
use edge_transport_http_egress_breaker_std::StdBreakerFactory;

/// @covers: describe
#[test]
fn test_describe_request_produces_a_non_empty_label_happy() {
    let processor = StdBreakerFactory.create_processor();
    let resp = processor.describe(DescribeRequest).expect("infallible");
    assert!(!resp.value.is_empty());
}

/// @covers: describe
#[test]
fn test_describe_request_is_reusable_across_calls_edge() {
    let processor = StdBreakerFactory.create_processor();
    let a = processor.describe(DescribeRequest).expect("infallible");
    let b = processor.describe(DescribeRequest).expect("infallible");
    assert_eq!(a.value, b.value);
}
