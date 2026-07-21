//! `Validator` — configuration validation contract.

use crate::api::{ConfigValidationRequest, RateError};

/// Validation contract for rate-limiter configuration.
pub trait Validator: Send + Sync {
    /// Validate the rate configuration, returning `Ok(())` on success or
    /// an error describing which field is invalid.
    fn validate(&self, request: ConfigValidationRequest) -> Result<(), RateError>;
}
