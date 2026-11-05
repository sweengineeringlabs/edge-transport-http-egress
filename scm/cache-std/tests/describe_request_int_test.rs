//! Integration tests for `DescribeRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache_api::{CacheFactory, DescribeRequest};
use edge_transport_http_egress_cache_std::StdCacheFactory;

/// @covers: describe
#[test]
fn test_describe_request_produces_a_non_empty_label_happy() {
    let resp = StdCacheFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("infallible");
    assert!(!resp.value.is_empty());
}

/// @covers: describe
#[test]
fn test_describe_request_is_reusable_across_calls_edge() {
    let a = StdCacheFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("infallible");
    let b = StdCacheFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("infallible");
    assert_eq!(a.value, b.value);
}
