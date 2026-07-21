//! Integration tests for the `RateLimitRequest` DTO.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::RateLimitRequest;

/// @covers: RateLimitRequest
#[test]
fn test_rate_limit_request_is_constructable_and_debuggable() {
    let req = RateLimitRequest;
    let cloned = req;
    assert_eq!(
        format!("{cloned:?}"),
        "RateLimitRequest",
        "RateLimitRequest Debug must name the unit struct"
    );
}
