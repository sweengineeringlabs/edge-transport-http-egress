//! Integration tests for `DescribeRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{DescribeRequest, ProcessorFactory};

/// @covers: describe
#[test]
fn test_describe_request_produces_a_non_empty_label_happy() {
    let processor = ProcessorFactory::create();
    let resp = processor.describe(DescribeRequest).expect("infallible");
    assert!(!resp.value.is_empty());
}

/// @covers: describe
#[test]
fn test_describe_request_is_reusable_across_calls_edge() {
    let processor = ProcessorFactory::create();
    let a = processor.describe(DescribeRequest).expect("infallible");
    let b = processor.describe(DescribeRequest).expect("infallible");
    assert_eq!(a.value, b.value);
}
