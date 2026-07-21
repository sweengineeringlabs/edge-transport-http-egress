//! `Validator` — configuration validation contract.

use crate::api::{CassetteError, ConfigValidationRequest};

/// Validation contract for cassette configuration.
pub trait Validator: Send + Sync {
    /// Validate the cassette configuration, returning `Ok(())` on success or
    /// an error describing which field is invalid.
    fn validate(&self, request: ConfigValidationRequest) -> Result<(), CassetteError>;
}
