//! Integration tests for `ValidateRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport_api::{
    AlwaysValidConfig, TransportFactory, ValidateRequest, Validator, ValidatorObject,
};
use edge_transport_http_egress_transport_std::StdTransportFactory;

/// `ValidateRequest` must actually reach the validator's `validate` method —
/// a config that's always valid must pass through the SAF gateway function,
/// while a config that always rejects must fail through the very same call
/// path, proving the gateway isn't hardcoded to always succeed.
#[test]
fn test_validate_request_reaches_validator_via_saf_gateway_happy() {
    assert!(StdTransportFactory.validate(&AlwaysValidConfig).is_ok());

    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(
            &self,
            _request: ValidateRequest,
        ) -> Result<(), edge_transport_http_egress_transport_api::ValidationError> {
            Err(
                edge_transport_http_egress_transport_api::ValidationError::Invalid(
                    "always rejects".to_string(),
                ),
            )
        }
    }
    assert!(
        StdTransportFactory.validate(&AlwaysInvalid).is_err(),
        "a rejecting validator must surface as Err through the same gateway"
    );
}

/// `ValidatorObject` (the dyn-safe alias `dyn Validator`) must be usable as a
/// real trait object: boxing a concrete validator behind it and calling
/// `validate` through the alias must dispatch to the concrete impl — proven
/// by pairing an always-ok and an always-err implementor behind the same
/// alias and observing both outcomes.
#[test]
fn test_validate_request_validator_object_alias_dispatches_edge() {
    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(
            &self,
            _request: ValidateRequest,
        ) -> Result<(), edge_transport_http_egress_transport_api::ValidationError> {
            Err(
                edge_transport_http_egress_transport_api::ValidationError::Invalid(
                    "always rejects".to_string(),
                ),
            )
        }
    }

    let ok_boxed: Box<ValidatorObject> = Box::new(AlwaysValidConfig);
    let err_boxed: Box<ValidatorObject> = Box::new(AlwaysInvalid);
    assert!(ok_boxed.validate(ValidateRequest).is_ok());
    assert!(
        err_boxed.validate(ValidateRequest).is_err(),
        "a rejecting implementor behind the same dyn-safe alias must dispatch to Err"
    );
}
