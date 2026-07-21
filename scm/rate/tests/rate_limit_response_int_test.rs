//! Integration tests for the `RateLimitResponse` DTO.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::RateLimitResponse;

/// @covers: RateLimitResponse
#[test]
fn test_rate_limit_response_carries_tokens_per_second() {
    let resp = RateLimitResponse {
        tokens_per_second: 250,
    };
    assert_eq!(
        resp.tokens_per_second, 250,
        "RateLimitResponse must carry the configured refill rate"
    );
}
