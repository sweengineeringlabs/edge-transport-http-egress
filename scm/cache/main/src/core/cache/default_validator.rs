//! `impl Validator for DefaultValidator` — cache config validation.

use crate::api::{CacheError, ConfigValidationRequest, Validator};

/// Default [`Validator`] implementation for [`CacheConfig`](crate::api::CacheConfig).
pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(&self, request: ConfigValidationRequest) -> Result<(), CacheError> {
        let config = request.config;
        if config.max_entries == 0 {
            return Err(CacheError::InvalidConfig(
                "max_entries must be >= 1".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::CacheConfig;

    /// @covers: validate
    #[test]
    fn test_validate_accepts_default_config() {
        // A valid config passes...
        assert!(DefaultValidator
            .validate(ConfigValidationRequest {
                config: CacheConfig::default()
            })
            .is_ok());
        // ...and a single field flipped to invalid on an otherwise-valid config
        // must fail — proving the Ok above isn't unconditional.
        let invalid = CacheConfig {
            max_entries: 0,
            ..CacheConfig::default()
        };
        assert!(DefaultValidator
            .validate(ConfigValidationRequest { config: invalid })
            .is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_zero_max_entries() {
        let cfg = CacheConfig {
            max_entries: 0,
            ..CacheConfig::default()
        };
        let err = DefaultValidator
            .validate(ConfigValidationRequest { config: cfg })
            .expect_err("zero max_entries must be rejected");
        assert!(matches!(err, CacheError::InvalidConfig(_)));
    }
}
