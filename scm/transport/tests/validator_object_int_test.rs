//! Integration tests for `ValidatorObject` (`dyn Validator`).
//!
//! `ValidatorObject` is the public dyn-safe alias for the `Validator` trait.
//! These tests dispatch real validation calls through the alias so a stub that
//! ignored the concrete implementation would be caught.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{
    AlwaysValidConfig, HttpConfig, ValidatableHttpConfig, ValidateRequest, ValidationError,
    ValidatorObject,
};

/// @covers: ValidatorObject
#[test]
fn transport_struct_validator_object_alias_is_accessible_int_test() {
    // A passing config dispatched through the `dyn Validator` alias returns Ok...
    let ok: &ValidatorObject = &AlwaysValidConfig;
    assert!(
        ok.validate(ValidateRequest).is_ok(),
        "AlwaysValidConfig must pass validation"
    );

    // ...and a zero-timeout config dispatched through the same alias returns the
    // concrete impl's real error, proving the call routes to the implementation.
    let bad = ValidatableHttpConfig {
        config: HttpConfig {
            timeout_secs: 0,
            ..HttpConfig::default()
        },
    };
    let bad_ref: &ValidatorObject = &bad;
    let err = bad_ref
        .validate(ValidateRequest)
        .expect_err("zero timeout_secs must fail validation");
    assert!(
        matches!(err, ValidationError::Invalid(ref m) if m.contains("timeout_secs")),
        "unexpected error: {err}"
    );
}

/// @covers: ValidatorObject object safety
#[test]
fn transport_struct_validator_object_is_object_safe_int_test() {
    // Two distinct concrete impls coerced to the same `dyn Validator` alias must
    // each dispatch to their own behaviour.
    let good = ValidatableHttpConfig {
        config: HttpConfig::default(),
    };
    let good_ref: &ValidatorObject = &good;
    assert!(
        good_ref.validate(ValidateRequest).is_ok(),
        "default config must validate through the alias"
    );

    let bad = ValidatableHttpConfig {
        config: HttpConfig {
            connect_timeout_secs: 0,
            ..HttpConfig::default()
        },
    };
    let bad_ref: &ValidatorObject = &bad;
    let err = bad_ref
        .validate(ValidateRequest)
        .expect_err("zero connect_timeout_secs must fail validation");
    assert!(
        matches!(err, ValidationError::Invalid(ref m) if m.contains("connect_timeout_secs")),
        "unexpected error: {err}"
    );
}
