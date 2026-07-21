//! Integration tests for `DescribeRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{DescribeRequest, HttpCassetteSvc, Processor};

/// @covers: describe
#[test]
fn test_describe_request_produces_a_non_empty_label_happy() {
    let resp = HttpCassetteSvc
        .describe(DescribeRequest)
        .expect("infallible");
    assert!(!resp.value.is_empty());
}

/// @covers: describe
#[test]
fn test_describe_request_is_reusable_across_calls_edge() {
    let a = HttpCassetteSvc
        .describe(DescribeRequest)
        .expect("infallible");
    let b = HttpCassetteSvc
        .describe(DescribeRequest)
        .expect("infallible");
    assert_eq!(a.value, b.value);
}
