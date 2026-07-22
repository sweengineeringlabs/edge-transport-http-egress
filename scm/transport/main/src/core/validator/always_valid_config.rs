//! `impl Validator for AlwaysValidConfig` — the declaration lives in `api/`.

use crate::api::{AlwaysValidConfig, ValidateRequest, ValidationError, Validator};

impl Validator for AlwaysValidConfig {
    fn validate(&self, _request: ValidateRequest) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: validate
    #[test]
    fn test_validate_always_returns_ok() {
        // Strong equality (not just is_ok()) proves the exact Ok(()) payload,
        // and a second independent call proves the marker is stateless.
        assert_eq!(AlwaysValidConfig.validate(ValidateRequest), Ok(()));
        assert_eq!(AlwaysValidConfig.validate(ValidateRequest), Ok(()));
    }
}
