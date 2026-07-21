//! `impl Validator for DefaultValidator` — circuit-breaker config validation.

use crate::api::{BreakerError, ConfigValidationRequest, Validator};

/// Default [`Validator`] implementation for [`BreakerConfig`](crate::api::BreakerConfig).
pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(&self, request: ConfigValidationRequest) -> Result<(), BreakerError> {
        let config = request.config;
        if config.failure_threshold == 0 {
            return Err(BreakerError::InvalidConfig(
                "failure_threshold must be >= 1".to_string(),
            ));
        }
        if config.reset_after_successes == 0 {
            return Err(BreakerError::InvalidConfig(
                "reset_after_successes must be >= 1".to_string(),
            ));
        }
        if config.failure_statuses.is_empty() {
            return Err(BreakerError::InvalidConfig(
                "failure_statuses must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::BreakerConfig;

    fn valid() -> BreakerConfig {
        BreakerConfig {
            failure_threshold: 3,
            half_open_after_seconds: 10,
            reset_after_successes: 1,
            failure_statuses: vec![500],
        }
    }

    #[test]
    fn test_validate_valid_config_returns_ok() {
        assert!(DefaultValidator
            .validate(ConfigValidationRequest { config: valid() })
            .is_ok());
        // Sibling negative case in the same test: a single field flipped to
        // invalid on an otherwise-valid config must fail, proving is_ok()
        // above isn't just a stub that always succeeds regardless of input.
        let mut invalid = valid();
        invalid.failure_threshold = 0;
        assert!(DefaultValidator
            .validate(ConfigValidationRequest { config: invalid })
            .is_err());
    }

    #[test]
    fn test_validate_rejects_zero_failure_threshold() {
        let mut cfg = valid();
        cfg.failure_threshold = 0;
        let err = DefaultValidator
            .validate(ConfigValidationRequest { config: cfg })
            .unwrap_err();
        assert!(matches!(err, BreakerError::InvalidConfig(_)));
    }

    #[test]
    fn test_validate_rejects_zero_reset_after_successes() {
        let mut cfg = valid();
        cfg.reset_after_successes = 0;
        let err = DefaultValidator
            .validate(ConfigValidationRequest { config: cfg })
            .unwrap_err();
        assert!(matches!(err, BreakerError::InvalidConfig(_)));
    }

    #[test]
    fn test_validate_rejects_empty_failure_statuses() {
        let mut cfg = valid();
        cfg.failure_statuses = vec![];
        let err = DefaultValidator
            .validate(ConfigValidationRequest { config: cfg })
            .unwrap_err();
        assert!(matches!(err, BreakerError::InvalidConfig(_)));
    }
}
