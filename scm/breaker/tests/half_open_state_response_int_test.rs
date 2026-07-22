//! Integration tests for `HalfOpenStateResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{HalfOpenStateRequest, HostBreakerFactory};

/// @covers: HalfOpenStateResponse
#[test]
fn test_half_open_state_response_value_is_false_for_fresh_node_happy() {
    let node = HostBreakerFactory::create();
    let resp = node.is_half_open(HalfOpenStateRequest).expect("infallible");
    assert!(!resp.value);
}

/// @covers: HalfOpenStateResponse
#[test]
fn test_half_open_state_response_value_is_a_plain_bool_edge() {
    let node = HostBreakerFactory::create();
    let resp = node.is_half_open(HalfOpenStateRequest).expect("infallible");
    let described = if resp.value {
        "half-open"
    } else {
        "not half-open"
    };
    assert_eq!(described, "not half-open");
}
