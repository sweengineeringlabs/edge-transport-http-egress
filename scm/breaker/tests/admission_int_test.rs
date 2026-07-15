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
/// the mutex-protected state machine.
#[test]
fn breaker_enum_admission_is_copy_int_test() {
    fn require_copy<T: Copy>(_: T) {}
    require_copy(Admission::Proceed);
    require_copy(Admission::RejectOpen);
}

/// @covers: Admission
/// Confirms `Admission::Proceed` is the default path (equality with itself).
#[test]
fn breaker_enum_admission_proceed_equals_itself_int_test() {
    assert_eq!(Admission::Proceed, Admission::Proceed);
}
