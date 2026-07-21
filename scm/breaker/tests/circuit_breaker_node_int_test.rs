//! Integration tests for the `CircuitBreakerNode` trait.
//!
//! The crate's own implementor (`HostBreaker`) is `pub(crate)`, so it can't
//! be exercised from here. Instead, this proves `CircuitBreakerNode` is
//! genuinely usable by external consumers — the actual contract of
//! exporting a public trait — by implementing it locally.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    Admission, AdmitRequest, AdmitResponse, BreakerConfig, BreakerError, CircuitBreakerNode,
    Outcome, RecordRequest, RecordResponse,
};
use std::sync::Arc;

/// A minimal external node: always proceeds, and remembers the last outcome
/// recorded — enough to prove real dispatch through the trait, not just
/// that it compiles.
struct TestNode {
    last_outcome: Option<Outcome>,
}

impl CircuitBreakerNode for TestNode {
    fn admit(&mut self, _request: AdmitRequest) -> Result<AdmitResponse, BreakerError> {
        Ok(AdmitResponse {
            admission: Admission::Proceed,
        })
    }

    fn record(&mut self, request: RecordRequest) -> Result<RecordResponse, BreakerError> {
        self.last_outcome = Some(request.outcome);
        Ok(RecordResponse)
    }
}

/// @covers: CircuitBreakerNode
#[test]
fn test_circuit_breaker_node_trait_is_defined() {
    let mut node = TestNode { last_outcome: None };
    let config = Arc::new(BreakerConfig::default());
    let resp = node
        .admit(AdmitRequest {
            config: Arc::clone(&config),
        })
        .expect("infallible");
    assert_eq!(
        resp.admission,
        Admission::Proceed,
        "an external CircuitBreakerNode impl must dispatch to its own admit()"
    );
    node.record(RecordRequest {
        config,
        outcome: Outcome::Failure,
    })
    .expect("infallible");
    assert_eq!(
        node.last_outcome,
        Some(Outcome::Failure),
        "an external CircuitBreakerNode impl must dispatch to its own record()"
    );
}
