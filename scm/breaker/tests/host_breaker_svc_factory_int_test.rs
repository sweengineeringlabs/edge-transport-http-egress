//! Integration tests for [`HostBreakerFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    ClosedStateRequest, HalfOpenStateRequest, HostBreakerFactory, OpenStateRequest,
};

/// @covers: create
#[test]
fn test_create_produces_a_node_starting_closed_happy() {
    let node = HostBreakerFactory::create();
    let resp = node
        .is_closed(ClosedStateRequest)
        .expect("factory-produced node must succeed");
    assert!(resp.value, "a freshly created node must start Closed");
}

/// @covers: create
#[test]
fn test_create_produces_a_node_not_open_error() {
    let node = HostBreakerFactory::create();
    let resp = node.is_open(OpenStateRequest).expect("must succeed");
    assert!(!resp.value, "a freshly created node must not start Open");
}

/// @covers: create
#[test]
fn test_create_produces_a_node_not_half_open_edge() {
    let node = HostBreakerFactory::create();
    let resp = node
        .is_half_open(HalfOpenStateRequest)
        .expect("must succeed");
    assert!(
        !resp.value,
        "a freshly created node must not start HalfOpen"
    );
}
