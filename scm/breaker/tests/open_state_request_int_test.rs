//! Integration tests for `OpenStateRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerError, ClosedStateRequest, ClosedStateResponse, HalfOpenStateRequest,
    HalfOpenStateResponse, HostBreaker, HostBreakerFactory, OpenStateRequest, OpenStateResponse,
};

/// @covers: is_open
#[test]
fn test_is_open_freshly_created_node_returns_false_happy() {
    let node = HostBreakerFactory::create();
    let resp = node.is_open(OpenStateRequest).expect("infallible");
    assert!(!resp.value);
}

/// A minimal external test-double: `HostBreaker`'s real implementor is
/// `pub(crate)`, so an Open-state trip can't be driven through the SAF
/// factory alone (it only exposes read-only accessors). This proves the
/// trait's `is_open` contract dispatches correctly to a real Open state for
/// any external implementor.
struct TestHostBreaker {
    open: bool,
}

impl HostBreaker for TestHostBreaker {
    fn is_open(&self, _request: OpenStateRequest) -> Result<OpenStateResponse, BreakerError> {
        Ok(OpenStateResponse { value: self.open })
    }

    fn is_half_open(
        &self,
        _request: HalfOpenStateRequest,
    ) -> Result<HalfOpenStateResponse, BreakerError> {
        Ok(HalfOpenStateResponse { value: false })
    }

    fn is_closed(&self, _request: ClosedStateRequest) -> Result<ClosedStateResponse, BreakerError> {
        Ok(ClosedStateResponse { value: !self.open })
    }
}

/// @covers: is_open
#[test]
fn test_is_open_tripped_node_returns_true_error() {
    let node = TestHostBreaker { open: true };
    let resp = node.is_open(OpenStateRequest).expect("infallible");
    assert!(
        resp.value,
        "an external HostBreaker impl reporting Open must dispatch through is_open as true"
    );
}
