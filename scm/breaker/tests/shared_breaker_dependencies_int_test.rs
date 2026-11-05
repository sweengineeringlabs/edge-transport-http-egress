//! Integration coverage for the protocol-agnostic breaker crates.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_breaker::DefaultBreakerTransition;
use edge_transport_breaker_policy::{
    Admission, AdmitRequest, BreakerConfig, BreakerState, BreakerTransition, Outcome,
    RecordOutcomeRequest,
};

/// @covers: edge_transport_breaker
/// @covers: edge_transport_breaker_policy
#[test]
fn breaker_type_shared_breaker_transition_records_failure_int_test() {
    let transition = DefaultBreakerTransition;
    let config = BreakerConfig {
        failure_threshold: 2,
        cool_down_seconds: 10,
        half_open_probe_count: 1,
    };
    let admitted = transition
        .admit(AdmitRequest {
            state: BreakerState::Closed,
            consecutive_failures: 0,
            consecutive_successes: 0,
            config,
        })
        .expect("closed breaker must admit the request");

    assert_eq!(admitted.admission, Admission::Proceed);
    assert_eq!(admitted.state, BreakerState::Closed);

    let recorded = transition
        .record(RecordOutcomeRequest {
            state: admitted.state,
            consecutive_failures: admitted.consecutive_failures,
            consecutive_successes: admitted.consecutive_successes,
            config,
            outcome: Outcome::Failure,
        })
        .expect("recording a request outcome must succeed");

    assert_eq!(recorded.state, BreakerState::Closed);
    assert_eq!(recorded.consecutive_failures, 1);
}
