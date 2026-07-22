//! Integration tests for `AdmitRequest`.

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

/// @covers: AdmitRequest
#[test]
fn test_admit_request_config_field_drives_closed_decision_happy() {
    let mut node = CircuitBreakerNodeFactory::create();
    let resp = node
        .admit(AdmitRequest { config: cfg() })
        .expect("infallible");
    assert_eq!(resp.admission, Admission::Proceed);
}

/// @covers: AdmitRequest
#[test]
fn test_admit_request_config_field_drives_reject_after_trip_error() {
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
        "AdmitRequest's config.failure_threshold must gate the reject decision"
    );
}

/// @covers: AdmitRequest
#[test]
fn test_admit_request_accepts_a_freshly_cloned_config_edge() {
    // A newly Arc::clone'd config (not the exact same Arc instance held
    // elsewhere) must still be honored identically.
    let shared = cfg();
    let mut node = CircuitBreakerNodeFactory::create();
    let resp = node
        .admit(AdmitRequest {
            config: Arc::clone(&shared),
        })
        .expect("infallible");
    assert_eq!(resp.admission, Admission::Proceed);
}
