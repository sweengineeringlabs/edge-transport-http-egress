//! `impl Validator for DefaultValidator` — cassette config validation.

use crate::api::{CassetteError, ConfigValidationRequest, Validator};

const VALID_MODES: [&str; 4] = ["replay", "record", "auto", "disabled"];

/// Default [`Validator`] implementation for [`CassetteConfig`](crate::api::CassetteConfig).
pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(&self, request: ConfigValidationRequest) -> Result<(), CassetteError> {
        let config = request.config;
        if !VALID_MODES.contains(&config.mode.as_str()) {
            return Err(CassetteError::InvalidConfig(format!(
                "mode must be one of {VALID_MODES:?}, got {:?}",
                config.mode
            )));
        }
        if config.cassette_dir.trim().is_empty() {
            return Err(CassetteError::InvalidConfig(
                "cassette_dir must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::CassetteConfig;

    fn valid() -> CassetteConfig {
        CassetteConfig::swe_default().expect("baseline parses")
    }

    /// @covers: validate
    #[test]
    fn test_validate_accepts_default_config() {
        assert!(DefaultValidator
            .validate(ConfigValidationRequest { config: valid() })
            .is_ok());
        // Sibling negative case: a single field flipped to invalid on an
        // otherwise-valid config must fail, proving the Ok above isn't
        // unconditional.
        let mut invalid = valid();
        invalid.mode = "bogus".to_string();
        assert!(DefaultValidator
            .validate(ConfigValidationRequest { config: invalid })
            .is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_unknown_mode() {
        let mut cfg = valid();
        cfg.mode = "bogus".to_string();
        let err = DefaultValidator
            .validate(ConfigValidationRequest { config: cfg })
            .expect_err("unknown mode must be rejected");
        assert!(matches!(err, CassetteError::InvalidConfig(_)));
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_empty_cassette_dir() {
        let mut cfg = valid();
        cfg.cassette_dir = "  ".to_string();
        let err = DefaultValidator
            .validate(ConfigValidationRequest { config: cfg })
            .expect_err("empty cassette_dir must be rejected");
        assert!(matches!(err, CassetteError::InvalidConfig(_)));
    }
}
