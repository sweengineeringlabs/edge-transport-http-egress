//! Integration tests for `ClosedStateResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{ClosedStateRequest, HostBreakerFactory};

/// @covers: ClosedStateResponse
#[test]
fn test_closed_state_response_value_is_true_for_fresh_node_happy() {
    let node = HostBreakerFactory::create();
    let resp = node.is_closed(ClosedStateRequest).expect("infallible");
    assert!(resp.value);
}

/// @covers: ClosedStateResponse
#[test]
fn test_closed_state_response_value_is_a_plain_bool_edge() {
    let node = HostBreakerFactory::create();
    let resp = node.is_closed(ClosedStateRequest).expect("infallible");
    let described = if resp.value { "closed" } else { "not closed" };
    assert_eq!(described, "closed");
}
