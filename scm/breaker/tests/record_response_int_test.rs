//! Integration tests for `RecordResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    Admission, AdmitRequest, BreakerConfig, CircuitBreakerNodeFactory, Outcome, RecordRequest,
};
use std::sync::Arc;

fn cfg() -> Arc<BreakerConfig> {
    Arc::new(BreakerConfig {
        failure_threshold: 1,
        half_open_after_seconds: 30,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    })
}

/// @covers: RecordResponse
#[test]
fn test_record_response_is_returned_on_success_happy() {
    let mut node = CircuitBreakerNodeFactory::create();
    let result = node.record(RecordRequest {
        config: cfg(),
        outcome: Outcome::Success,
    });
    assert!(result.is_ok(), "record must return Ok(RecordResponse)");
    // Real payload proof: recording Success on a fresh (Closed) node must
    // not trip the breaker — is_ok() alone wouldn't catch a stub record()
    // that silently trips regardless of outcome.
    let admit_resp = node
        .admit(AdmitRequest { config: cfg() })
        .expect("infallible");
    assert_eq!(admit_resp.admission, Admission::Proceed);
}

/// @covers: RecordResponse
#[test]
fn test_record_response_is_returned_regardless_of_outcome_edge() {
    let mut node = CircuitBreakerNodeFactory::create();
    let result = node.record(RecordRequest {
        config: cfg(),
        outcome: Outcome::Failure,
    });
    assert!(
        result.is_ok(),
        "RecordResponse must be produced for Failure outcomes too, not only Success"
    );
    // Real payload proof: a Failure outcome at threshold=1 must actually
    // trip the breaker — proves record() genuinely applied the outcome
    // rather than being a no-op stub that always returns Ok.
    let admit_resp = node
        .admit(AdmitRequest { config: cfg() })
        .expect("infallible");
    assert_eq!(admit_resp.admission, Admission::RejectOpen);
}
