//! Integration tests for the `DescribeRequest` DTO.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::DescribeRequest;

/// @covers: DescribeRequest
#[test]
fn test_describe_request_is_constructable_and_debuggable() {
    let req = DescribeRequest;
    // A round-trip through Clone/Debug proves the DTO derives are wired.
    let cloned = req;
    assert_eq!(
        format!("{cloned:?}"),
        "DescribeRequest",
        "DescribeRequest Debug must name the unit struct"
    );
}
