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
/// into `CircuitBreakerNode::record`: using the original after copying it
/// into another binding must not be a move error, and both must still
/// compare equal to the same variant.
#[test]
fn breaker_enum_outcome_is_copy_int_test() {
    let original = Outcome::Success;
    let copy = original;
    assert_eq!(
        original,
        Outcome::Success,
        "original must remain usable after the copy (would be a move error otherwise)"
    );
    assert_eq!(copy, original, "the copy must equal the original");
}
