//! Integration tests for `ClosedStateRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerError, ClosedStateRequest, ClosedStateResponse, HalfOpenStateRequest,
    HalfOpenStateResponse, HostBreaker, HostBreakerFactory, OpenStateRequest, OpenStateResponse,
};

/// @covers: is_closed
#[test]
fn test_is_closed_freshly_created_node_returns_true_happy() {
    let node = HostBreakerFactory::create();
    let resp = node.is_closed(ClosedStateRequest).expect("infallible");
    assert!(resp.value);
}

/// A minimal external test-double — see `open_state_request_int_test.rs` for
/// why `HostBreaker`'s real implementor can't drive a non-Closed state
/// through the SAF factory alone.
struct TestHostBreaker {
    closed: bool,
}

impl HostBreaker for TestHostBreaker {
    fn is_open(&self, _request: OpenStateRequest) -> Result<OpenStateResponse, BreakerError> {
        Ok(OpenStateResponse { value: false })
    }

    fn is_half_open(
        &self,
        _request: HalfOpenStateRequest,
    ) -> Result<HalfOpenStateResponse, BreakerError> {
        Ok(HalfOpenStateResponse {
            value: !self.closed,
        })
    }

    fn is_closed(&self, _request: ClosedStateRequest) -> Result<ClosedStateResponse, BreakerError> {
        Ok(ClosedStateResponse { value: self.closed })
    }
}

/// @covers: is_closed
#[test]
fn test_is_closed_half_open_node_returns_false_error() {
    let node = TestHostBreaker { closed: false };
    let resp = node.is_closed(ClosedStateRequest).expect("infallible");
    assert!(
        !resp.value,
        "an external HostBreaker impl reporting non-Closed must dispatch through is_closed as false"
    );
}
