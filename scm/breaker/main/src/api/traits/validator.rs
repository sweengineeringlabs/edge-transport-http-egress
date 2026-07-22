//! `Validator` — configuration validation contract.

use crate::api::{BreakerError, ConfigValidationRequest};

/// Validation contract for circuit-breaker configuration.
pub trait Validator: Send + Sync {
    /// Validate the breaker configuration, returning `Ok(())` on success or
    /// an error describing which field is invalid.
    fn validate(&self, request: ConfigValidationRequest) -> Result<(), BreakerError>;
}
