//! Integration tests for [`ProcessorFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{DescribeRequest, ProcessorFactory};

/// @covers: create
#[test]
fn test_create_produces_a_working_processor_happy() {
    let processor = ProcessorFactory::create();
    let resp = processor.describe(DescribeRequest).expect("infallible");
    assert!(!resp.value.is_empty());
}

/// @covers: create
#[test]
fn test_create_identifies_as_http_cassette_error() {
    let processor = ProcessorFactory::create();
    let resp = processor.describe(DescribeRequest).expect("must succeed");
    assert!(resp.value.contains("cassette"));
}

/// @covers: create
#[test]
fn test_create_produces_independent_instances_edge() {
    let first = ProcessorFactory::create();
    let second = ProcessorFactory::create();
    let resp1 = first.describe(DescribeRequest).expect("first must succeed");
    let resp2 = second
        .describe(DescribeRequest)
        .expect("second must succeed");
    assert_eq!(resp1.value, resp2.value);
}
