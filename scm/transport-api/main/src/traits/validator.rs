//! `Validator` — outbound configuration validation contract.

use crate::ValidateRequest;
use crate::ValidationError;

/// Validates an outbound configuration or request value.
pub trait Validator {
    /// Returns `Ok(())` when the value is valid, or a [`ValidationError`]
    /// describing why it is not.
    fn validate(&self, request: ValidateRequest) -> Result<(), ValidationError>;
}
