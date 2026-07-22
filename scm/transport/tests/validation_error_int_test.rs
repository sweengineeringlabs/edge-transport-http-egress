//! Integration tests for `ValidationError`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{
    HttpConfig, ValidatableHttpConfig, ValidateRequest, ValidationError, Validator,
};

/// `ValidationError::Invalid` must carry the actual reason string through to
/// the caller, not a generic/placeholder message; a valid config, by
/// contrast, must not produce a `ValidationError` at all — proving the
/// Err case above isn't a validator that always rejects.
#[test]
fn test_validation_error_invalid_carries_reason_happy() {
    let cfg = ValidatableHttpConfig {
        config: HttpConfig {
            timeout_secs: 0,
            ..HttpConfig::default()
        },
    };
    let err = cfg
        .validate(ValidateRequest)
        .expect_err("zero timeout_secs must be rejected");

    let ValidationError::Invalid(reason) = err;
    assert!(
        reason.contains("timeout_secs"),
        "ValidationError::Invalid must name the offending field, got: {reason}"
    );

    let valid_cfg = ValidatableHttpConfig {
        config: HttpConfig::default(),
    };
    assert!(
        valid_cfg.validate(ValidateRequest).is_ok(),
        "a valid config must not raise ValidationError"
    );
}

/// Two independently constructed `Invalid` reasons must compare equal iff
/// their messages match — proving `PartialEq` is derived on content, not
/// identity.
#[test]
fn test_validation_error_equality_is_content_based_edge() {
    let a = ValidationError::Invalid("same".to_string());
    let b = ValidationError::Invalid("same".to_string());
    let c = ValidationError::Invalid("different".to_string());
    assert_eq!(a, b);
    assert_ne!(a, c);
}
