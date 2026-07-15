//! Integration tests for `api/types/breaker/outcome.rs`.
//! @covers: src/api/types/breaker/outcome.rs

use edge_transport_http_egress_breaker::Outcome;

/// @covers: Outcome
/// Confirms `Success` and `Failure` are distinct variants — the breaker
/// state machine transitions differ based on this equality check.
#[test]
fn breaker_enum_outcome_success_ne_failure_int_test() {
    assert_ne!(
        Outcome::Success,
        Outcome::Failure,
        "Success and Failure must be distinct"
    );
}

/// @covers: Outcome
/// Confirms `Outcome` supports `Copy` semantics — it is passed by value
/// into `CircuitBreakerNode::record`.
#[test]
fn breaker_enum_outcome_is_copy_int_test() {
    fn require_copy<T: Copy>(_: T) {}
    require_copy(Outcome::Success);
    require_copy(Outcome::Failure);
}

/// @covers: Outcome
/// Confirms `Outcome::Success` equals itself.
#[test]
fn breaker_enum_outcome_success_equals_itself_int_test() {
    assert_eq!(Outcome::Success, Outcome::Success);
}
