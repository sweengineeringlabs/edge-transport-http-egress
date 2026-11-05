//! `BreakerConfig` converts into the shared `edge-transport-breaker-policy` config the
//! state machine consumes, and the contract re-exports that crate's circuit-state types.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_breaker_policy::{Admission as PolicyAdmission, BreakerConfig as PolicyConfig};
use edge_transport_http_egress_breaker_api::{Admission, BreakerConfig};

/// @covers: from
#[test]
fn test_from_maps_every_field_to_the_policy_config_happy() {
    let config = BreakerConfig {
        failure_threshold: 4,
        half_open_after_seconds: 45,
        reset_after_successes: 6,
        failure_statuses: vec![500],
    };
    let policy = PolicyConfig::from(config);
    assert_eq!(policy.failure_threshold, 4);
    assert_eq!(
        policy.cool_down_seconds, 45,
        "half_open_after_seconds is the cool-down"
    );
    assert_eq!(
        policy.half_open_probe_count, 6,
        "reset_after_successes is the probe count"
    );
}

/// @covers: from
#[test]
fn test_from_the_default_config_keeps_the_documented_defaults_edge() {
    let policy = PolicyConfig::from(BreakerConfig::default());
    assert_eq!(
        (
            policy.failure_threshold,
            policy.cool_down_seconds,
            policy.half_open_probe_count
        ),
        (5, 30, 3)
    );
}

/// @covers: Admission
#[test]
fn test_admission_is_the_policy_crates_type_happy() {
    let admitted: PolicyAdmission = Admission::Proceed;
    assert_eq!(admitted, PolicyAdmission::Proceed);
    assert_ne!(Admission::Proceed, Admission::RejectOpen);
}
