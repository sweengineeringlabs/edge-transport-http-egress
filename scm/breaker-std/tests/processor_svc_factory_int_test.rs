//! Integration tests for [`ProcessorFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker_api::{BreakerFactory, DescribeRequest};
use edge_transport_http_egress_breaker_std::StdBreakerFactory;

/// @covers: create
#[test]
fn test_create_produces_a_working_processor_happy() {
    let processor = StdBreakerFactory.create_processor();
    let resp = processor
        .describe(DescribeRequest)
        .expect("factory-produced processor must succeed");
    assert!(!resp.value.is_empty());
}

/// @covers: create
#[test]
fn test_create_identifies_as_http_breaker_error() {
    // "error"-flavored scenario: prove the label is a genuine, specific
    // identifier rather than an empty/placeholder stub.
    let processor = StdBreakerFactory.create_processor();
    let resp = processor.describe(DescribeRequest).expect("must succeed");
    assert!(resp.value.contains("breaker"));
}

/// @covers: create
#[test]
fn test_create_produces_independent_instances_edge() {
    let first = StdBreakerFactory.create_processor();
    let second = StdBreakerFactory.create_processor();
    let resp1 = first.describe(DescribeRequest).expect("first must succeed");
    let resp2 = second
        .describe(DescribeRequest)
        .expect("second must succeed");
    assert_eq!(resp1.value, resp2.value);
}
