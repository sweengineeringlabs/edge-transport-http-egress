//! Integration tests for `DescribeResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{DescribeRequest, ProcessorFactory};

/// @covers: DescribeResponse
#[test]
fn test_describe_response_value_identifies_the_loadbalancer_happy() {
    let processor = ProcessorFactory::create();
    let resp = processor.describe(DescribeRequest).expect("infallible");
    assert!(resp.value.contains("loadbalancer"));
}

/// @covers: DescribeResponse
#[test]
fn test_describe_response_value_is_an_owned_string_edge() {
    let processor = ProcessorFactory::create();
    let resp = processor.describe(DescribeRequest).expect("infallible");
    let value: String = resp.value;
    assert!(!value.is_empty());
}
