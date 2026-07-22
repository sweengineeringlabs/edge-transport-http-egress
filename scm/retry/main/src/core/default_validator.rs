//! `DefaultValidator` — the default [`Validator`] over a [`RetryConfig`].

use crate::api::RetryError;
use crate::api::ValidationRequest;
use crate::api::Validator;

/// Default configuration validator. Delegates to the field-range checks on
/// [`RetryConfig`](crate::api::RetryConfig) and surfaces failures as
/// [`RetryError::InvalidConfig`].
pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(&self, req: ValidationRequest) -> Result<(), RetryError> {
        req.config.validate().map_err(RetryError::InvalidConfig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::RetryConfig;

    /// @covers: validate
    #[test]
    fn test_validate_accepts_default_config() {
        let req = ValidationRequest {
            config: RetryConfig::default(),
        };
        assert!(DefaultValidator.validate(req).is_ok());
        // Sibling negative case in the same test: an otherwise-valid config
        // with multiplier=0 must fail, proving the is_ok() above isn't a
        // stub that always succeeds regardless of input.
        let invalid = ValidationRequest {
            config: RetryConfig {
                multiplier: 0.0,
                ..RetryConfig::default()
            },
        };
        assert!(DefaultValidator.validate(invalid).is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_zero_multiplier() {
        let req = ValidationRequest {
            config: RetryConfig {
                multiplier: 0.0,
                ..RetryConfig::default()
            },
        };
        let err = DefaultValidator
            .validate(req)
            .expect_err("multiplier=0 must be rejected");
        assert!(
            matches!(err, RetryError::InvalidConfig(_)),
            "must be InvalidConfig, got {err:?}"
        );
    }
}
