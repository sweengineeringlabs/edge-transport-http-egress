//! Integration tests for [`ProcessorSvcFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache::{DescribeRequest, ProcessorSvcFactory};

/// @covers: create
#[test]
fn test_create_produces_a_working_processor_happy() {
    let processor = ProcessorSvcFactory::create();
    let resp = processor.describe(DescribeRequest).expect("infallible");
    assert!(!resp.value.is_empty());
}

/// @covers: create
#[test]
fn test_create_identifies_as_http_cache_error() {
    let processor = ProcessorSvcFactory::create();
    let resp = processor.describe(DescribeRequest).expect("must succeed");
    assert!(resp.value.contains("cache"));
}

/// @covers: create
#[test]
fn test_create_produces_independent_instances_edge() {
    let first = ProcessorSvcFactory::create();
    let second = ProcessorSvcFactory::create();
    let resp1 = first.describe(DescribeRequest).expect("first must succeed");
    let resp2 = second
        .describe(DescribeRequest)
        .expect("second must succeed");
    assert_eq!(resp1.value, resp2.value);
}
