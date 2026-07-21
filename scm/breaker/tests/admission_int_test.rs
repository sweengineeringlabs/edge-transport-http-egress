//! Integration tests for `api/types/breaker/admission.rs`.
//! @covers: src/api/types/breaker/admission.rs

use edge_transport_http_egress_breaker::Admission;

/// @covers: Admission
/// Confirms `Proceed` and `RejectOpen` are distinct variants — the breaker
/// layer makes control-flow decisions based on this equality.
#[test]
fn breaker_enum_admission_proceed_ne_reject_open_int_test() {
    assert_ne!(
        Admission::Proceed,
        Admission::RejectOpen,
        "Proceed and RejectOpen must be distinct"
    );
}

/// @covers: Admission
/// Confirms `Admission` supports `Copy` semantics required for use inside
/// the mutex-protected state machine: using the original after copying it
/// into another binding must not be a move error, and both must still
/// compare equal to the same variant.
#[test]
fn breaker_enum_admission_is_copy_int_test() {
    let original = Admission::Proceed;
    let copy = original;
    assert_eq!(
        original,
        Admission::Proceed,
        "original must remain usable after the copy (would be a move error otherwise)"
    );
    assert_eq!(copy, original, "the copy must equal the original");
}
