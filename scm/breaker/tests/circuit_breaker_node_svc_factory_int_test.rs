//! Integration tests for [`CircuitBreakerNodeFactory`].

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

/// @covers: create
#[test]
fn test_create_produces_a_node_that_admits_when_closed_happy() {
    let mut node = CircuitBreakerNodeFactory::create();
    let resp = node
        .admit(AdmitRequest { config: cfg() })
        .expect("factory-produced node must succeed");
    assert_eq!(resp.admission, Admission::Proceed);
}

/// @covers: create
#[test]
fn test_create_produces_a_node_that_trips_open_on_failures_error() {
    let mut node = CircuitBreakerNodeFactory::create();
    for _ in 0..2 {
        node.record(RecordRequest {
            config: cfg(),
            outcome: Outcome::Failure,
        })
        .expect("must succeed");
    }
    let resp = node
        .admit(AdmitRequest { config: cfg() })
        .expect("must succeed");
    assert_eq!(
        resp.admission,
        Admission::RejectOpen,
        "node must reject once tripped open"
    );
}

/// @covers: create
#[test]
fn test_create_produces_independent_nodes_edge() {
    let mut a = CircuitBreakerNodeFactory::create();
    let mut b = CircuitBreakerNodeFactory::create();
    for _ in 0..2 {
        a.record(RecordRequest {
            config: cfg(),
            outcome: Outcome::Failure,
        })
        .expect("must succeed");
    }
    let resp_a = a
        .admit(AdmitRequest { config: cfg() })
        .expect("must succeed");
    let resp_b = b
        .admit(AdmitRequest { config: cfg() })
        .expect("must succeed");
    assert_eq!(resp_a.admission, Admission::RejectOpen, "a is tripped");
    assert_eq!(
        resp_b.admission,
        Admission::Proceed,
        "b is independent, still closed"
    );
}
