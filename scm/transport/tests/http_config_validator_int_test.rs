//! Integration tests for `HttpConfigValidator` (`dyn Validator`).
//!
//! `HttpConfigValidator` is the public dyn-safe alias for the `Validator`
//! trait applied to HTTP configuration. These tests dispatch real validation
//! calls through the alias, pinning both the pass and fail outcomes.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{
    HttpConfig, HttpConfigValidator, ValidatableHttpConfig, ValidateRequest, ValidationError,
};

/// @covers: HttpConfigValidator
#[test]
fn test_http_config_validator_type_is_object_safe() {
    let valid = ValidatableHttpConfig {
        config: HttpConfig::default(),
    };
    let obj: &HttpConfigValidator = &valid;
    assert!(
        obj.validate(ValidateRequest).is_ok(),
        "default config must validate through the dyn Validator alias"
    );

    let invalid = ValidatableHttpConfig {
        config: HttpConfig {
            timeout_secs: 0,
            ..HttpConfig::default()
        },
    };
    let obj2: &HttpConfigValidator = &invalid;
    let err = obj2
        .validate(ValidateRequest)
        .expect_err("zero timeout_secs must fail through the alias");
    assert!(
        matches!(err, ValidationError::Invalid(ref m) if m.contains("timeout_secs")),
        "unexpected error: {err}"
    );
}
