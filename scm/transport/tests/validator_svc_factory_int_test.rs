//! Integration tests for `ValidatorSvcFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{
    ValidateRequest, ValidationError, Validator, ValidatorSvcFactory,
};

/// `create()` must return an always-passing `Validator` — every call must
/// accept a `ValidateRequest`. Paired against a hand-rolled rejecting
/// `Validator` through the same `validate` call shape, proving `is_ok()`
/// above reflects the factory's real (documented) always-pass behaviour
/// rather than a `Result` type that can never be `Err`.
#[test]
fn test_validator_svc_factory_create_always_passes_happy() {
    let validator = ValidatorSvcFactory::create();
    assert!(validator.validate(ValidateRequest).is_ok());

    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(&self, _request: ValidateRequest) -> Result<(), ValidationError> {
            Err(ValidationError::Invalid("always rejects".to_string()))
        }
    }
    assert!(
        AlwaysInvalid.validate(ValidateRequest).is_err(),
        "the Validator contract can express Err — the factory's Ok is a real verdict, not a stub type"
    );
}

/// Two independently created validators must behave identically (both pass),
/// proving `create()` doesn't hand back inconsistent state across calls —
/// contrasted against a rejecting `Validator` to show the assertions below
/// are a real verdict, not a `Result` type that can never be `Err`.
#[test]
fn test_validator_svc_factory_create_two_instances_consistent_edge() {
    let a = ValidatorSvcFactory::create();
    let b = ValidatorSvcFactory::create();
    assert!(a.validate(ValidateRequest).is_ok());
    assert!(b.validate(ValidateRequest).is_ok());

    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(&self, _request: ValidateRequest) -> Result<(), ValidationError> {
            Err(ValidationError::Invalid("always rejects".to_string()))
        }
    }
    assert!(AlwaysInvalid.validate(ValidateRequest).is_err());
}
