//! Integration tests for `DescribeResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette_api::{CassetteFactory, DescribeRequest};
use edge_transport_http_egress_cassette_std::StdCassetteFactory;

/// @covers: DescribeResponse
#[test]
fn test_describe_response_value_identifies_the_cassette_happy() {
    let resp = StdCassetteFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("infallible");
    assert!(resp.value.contains("cassette"));
}

/// @covers: DescribeResponse
#[test]
fn test_describe_response_value_is_an_owned_string_edge() {
    let resp = StdCassetteFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("infallible");
    let value: String = resp.value;
    assert!(!value.is_empty());
}
