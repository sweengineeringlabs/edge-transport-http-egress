//! `impl Validator for DefaultValidator` — rate-limiter config validation.

use crate::api::{ConfigValidationRequest, RateError, Validator};

/// Default [`Validator`] implementation for [`RateConfig`](crate::api::RateConfig).
pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(&self, request: ConfigValidationRequest) -> Result<(), RateError> {
        let config = request.config;
        if config.tokens_per_second == 0 {
            return Err(RateError::InvalidConfig(
                "tokens_per_second must be >= 1; a rate of 0 would block all requests".to_string(),
            ));
        }
        if config.burst_capacity == 0 {
            return Err(RateError::InvalidConfig(
                "burst_capacity must be >= 1; a burst of 0 would deny every request".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::RateConfig;

    fn valid() -> RateConfig {
        RateConfig {
            tokens_per_second: 10,
            burst_capacity: 20,
            per_host: true,
        }
    }

    /// @covers: validate
    #[test]
    fn test_validate_valid_config_returns_ok() {
        assert!(DefaultValidator
            .validate(ConfigValidationRequest { config: valid() })
            .is_ok());
        // Sibling negative case in the same test: a single field flipped to
        // invalid on an otherwise-valid config must fail, proving is_ok()
        // above isn't just a stub that always succeeds regardless of input.
        let mut invalid = valid();
        invalid.tokens_per_second = 0;
        assert!(DefaultValidator
            .validate(ConfigValidationRequest { config: invalid })
            .is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_zero_tokens_per_second() {
        let mut cfg = valid();
        cfg.tokens_per_second = 0;
        let err = DefaultValidator
            .validate(ConfigValidationRequest { config: cfg })
            .unwrap_err();
        assert!(matches!(err, RateError::InvalidConfig(_)));
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_zero_burst_capacity() {
        let mut cfg = valid();
        cfg.burst_capacity = 0;
        let err = DefaultValidator
            .validate(ConfigValidationRequest { config: cfg })
            .unwrap_err();
        assert!(matches!(err, RateError::InvalidConfig(_)));
    }
}
