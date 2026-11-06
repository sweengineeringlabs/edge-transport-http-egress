//! Integration tests for `DescribeResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cache_api::{CacheFactory, DescribeRequest};
use edge_transport_http_egress_cache_std::StdCacheFactory;

/// @covers: DescribeResponse
#[test]
fn test_describe_response_value_identifies_the_cache_happy() {
    let resp = StdCacheFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("infallible");
    assert!(resp.value.contains("cache"));
}

/// @covers: DescribeResponse
#[test]
fn test_describe_response_value_is_an_owned_string_edge() {
    let resp = StdCacheFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("infallible");
    let value: String = resp.value;
    assert!(!value.is_empty());
}
