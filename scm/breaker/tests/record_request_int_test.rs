//! Integration tests for `RecordRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    Admission, AdmitRequest, BreakerConfig, CircuitBreakerNodeFactory, Outcome, RecordRequest,
};
use std::sync::Arc;

fn cfg() -> Arc<BreakerConfig> {
    Arc::new(BreakerConfig {
        failure_threshold: 2,
        half_open_after_seconds: 30,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    })
}

/// @covers: record
#[test]
fn test_record_outcome_success_keeps_breaker_closed_happy() {
    let mut node = CircuitBreakerNodeFactory::create();
    node.record(RecordRequest {
        config: cfg(),
        outcome: Outcome::Success,
    })
    .expect("infallible");
    let resp = node
        .admit(AdmitRequest { config: cfg() })
        .expect("infallible");
    assert_eq!(resp.admission, Admission::Proceed);
}

/// @covers: record
#[test]
fn test_record_outcome_failure_at_threshold_trips_open_error() {
    let mut node = CircuitBreakerNodeFactory::create();
    for _ in 0..2 {
        node.record(RecordRequest {
            config: cfg(),
            outcome: Outcome::Failure,
        })
        .expect("infallible");
    }
    let resp = node
        .admit(AdmitRequest { config: cfg() })
        .expect("infallible");
    assert_eq!(
        resp.admission,
        Admission::RejectOpen,
        "RecordRequest's outcome field must drive real state transitions, not be ignored"
    );
}

/// @covers: record
#[test]
fn test_record_request_config_field_is_reusable_across_calls_edge() {
    let shared_config = cfg();
    let mut node = CircuitBreakerNodeFactory::create();
    node.record(RecordRequest {
        config: Arc::clone(&shared_config),
        outcome: Outcome::Failure,
    })
    .expect("infallible");
    node.record(RecordRequest {
        config: Arc::clone(&shared_config),
        outcome: Outcome::Failure,
    })
    .expect("infallible");
    let resp = node
        .admit(AdmitRequest {
            config: shared_config,
        })
        .expect("infallible");
    assert_eq!(resp.admission, Admission::RejectOpen);
}
