//! Integration tests for `HalfOpenStateRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{HalfOpenStateRequest, HostBreakerFactory};

/// @covers: is_half_open
#[test]
fn test_is_half_open_freshly_created_node_returns_false_happy() {
    let node = HostBreakerFactory::create();
    let resp = node.is_half_open(HalfOpenStateRequest).expect("infallible");
    assert!(!resp.value);
}

/// @covers: is_half_open
#[test]
fn test_is_half_open_request_is_reusable_across_calls_edge() {
    let node = HostBreakerFactory::create();
    let a = node.is_half_open(HalfOpenStateRequest).expect("infallible");
    let b = node.is_half_open(HalfOpenStateRequest).expect("infallible");
    assert_eq!(a.value, b.value, "repeated queries must be deterministic");
}
