//! `impl Validator for ValidatableHttpConfig` — the declaration lives in `api/`.

use crate::api::{ValidatableHttpConfig, ValidateRequest, ValidationError, Validator};

impl Validator for ValidatableHttpConfig {
    fn validate(&self, _request: ValidateRequest) -> Result<(), ValidationError> {
        if self.config.timeout_secs == 0 {
            return Err(ValidationError::Invalid(
                "timeout_secs must be greater than 0".to_string(),
            ));
        }
        if self.config.connect_timeout_secs == 0 {
            return Err(ValidationError::Invalid(
                "connect_timeout_secs must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::HttpConfig;

    /// @covers: validate
    #[test]
    fn test_validate_rejects_zero_timeout_secs() {
        let cfg = ValidatableHttpConfig {
            config: HttpConfig {
                timeout_secs: 0,
                ..HttpConfig::default()
            },
        };
        let err = cfg.validate(ValidateRequest).expect_err("must be rejected");
        assert!(matches!(err, ValidationError::Invalid(_)));
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_zero_connect_timeout_secs() {
        let cfg = ValidatableHttpConfig {
            config: HttpConfig {
                connect_timeout_secs: 0,
                ..HttpConfig::default()
            },
        };
        let err = cfg.validate(ValidateRequest).expect_err("must be rejected");
        assert!(matches!(err, ValidationError::Invalid(_)));
    }

    /// @covers: validate
    #[test]
    fn test_validate_accepts_default_config() {
        let cfg = ValidatableHttpConfig {
            config: HttpConfig::default(),
        };
        assert!(cfg.validate(ValidateRequest).is_ok());
        // Sibling negative case in the same test: an otherwise-default config
        // with timeout_secs=0 must fail, proving the is_ok() above isn't a
        // stub that always succeeds regardless of input.
        let invalid = ValidatableHttpConfig {
            config: HttpConfig {
                timeout_secs: 0,
                ..HttpConfig::default()
            },
        };
        assert!(invalid.validate(ValidateRequest).is_err());
    }
}
