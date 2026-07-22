//! Integration tests for `OpenStateResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{HostBreakerFactory, OpenStateRequest};

/// @covers: OpenStateResponse
#[test]
fn test_open_state_response_value_is_false_for_fresh_node_happy() {
    let node = HostBreakerFactory::create();
    let resp = node.is_open(OpenStateRequest).expect("infallible");
    assert!(!resp.value);
}

/// @covers: OpenStateResponse
#[test]
fn test_open_state_response_value_is_a_plain_bool_edge() {
    let node = HostBreakerFactory::create();
    let resp = node.is_open(OpenStateRequest).expect("infallible");
    // The response's `value` field is a bool usable directly in a condition,
    // not wrapped in any further indirection.
    let described = if resp.value { "open" } else { "not open" };
    assert_eq!(described, "not open");
}
