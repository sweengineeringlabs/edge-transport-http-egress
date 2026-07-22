//! Integration tests for the `DescribeResponse` DTO.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::DescribeResponse;

/// @covers: DescribeResponse
#[test]
fn test_describe_response_carries_value_field() {
    let resp = DescribeResponse {
        value: "http-rate".to_string(),
    };
    assert_eq!(
        resp.value, "http-rate",
        "DescribeResponse must carry the label it was constructed with"
    );
}
