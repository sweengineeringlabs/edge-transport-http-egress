//! Concrete [`Validator`] implementation for [`HttpConfig`].

use crate::api::{HttpConfig, ValidateRequest, ValidationError, Validator};

/// Validates an [`HttpConfig`] value for production use.
pub(crate) struct HttpConfigValidator<'a> {
    config: &'a HttpConfig,
}

impl<'a> HttpConfigValidator<'a> {
    pub(crate) fn new(config: &'a HttpConfig) -> Self {
        Self { config }
    }
}

impl<'a> Validator for HttpConfigValidator<'a> {
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

    /// @covers: new
    #[test]
    fn test_new_creates_validator_from_config_reference() {
        // A validator built over a valid config passes; one built over a
        // zero-timeout config fails — proving `new` binds the referenced config,
        // not a default/ignored one.
        let valid = HttpConfig::default();
        assert!(
            HttpConfigValidator::new(&valid)
                .validate(ValidateRequest)
                .is_ok(),
            "validator over a valid config must pass"
        );

        let invalid = HttpConfig {
            timeout_secs: 0,
            ..HttpConfig::default()
        };
        assert!(
            HttpConfigValidator::new(&invalid)
                .validate(ValidateRequest)
                .is_err(),
            "validator over a zero-timeout config must fail"
        );
    }

    /// @covers: validate
    #[test]
    fn test_http_config_validator_ok_for_valid_config() {
        let cfg = HttpConfig::default();
        assert!(
            HttpConfigValidator::new(&cfg)
                .validate(ValidateRequest)
                .is_ok(),
            "default config must be accepted"
        );

        // Sibling negative: a zero connect-timeout must be rejected, so the Ok
        // above is a real verdict rather than a validator that always passes.
        let bad = HttpConfig {
            connect_timeout_secs: 0,
            ..HttpConfig::default()
        };
        assert!(
            HttpConfigValidator::new(&bad)
                .validate(ValidateRequest)
                .is_err(),
            "zero connect_timeout_secs must be rejected"
        );
    }

    /// @covers: validate
    #[test]
    fn test_http_config_validator_err_for_zero_timeout() {
        let cfg = HttpConfig {
            timeout_secs: 0,
            ..HttpConfig::default()
        };
        let v = HttpConfigValidator::new(&cfg);
        let err = v.validate(ValidateRequest).unwrap_err();
        assert!(matches!(err, ValidationError::Invalid(ref m) if m.contains("timeout_secs")));
    }

    /// @covers: validate
    #[test]
    fn test_http_config_validator_err_for_zero_connect_timeout() {
        let cfg = HttpConfig {
            connect_timeout_secs: 0,
            ..HttpConfig::default()
        };
        let v = HttpConfigValidator::new(&cfg);
        let err = v.validate(ValidateRequest).unwrap_err();
        assert!(
            matches!(err, ValidationError::Invalid(ref m) if m.contains("connect_timeout_secs"))
        );
    }
}
