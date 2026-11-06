//! `Validator` — configuration validation contract.

use crate::RetryError;
use crate::ValidationRequest;

/// Configuration validation contract.
///
/// Implemented by [`DefaultValidator`](crate::RetryConfig) validators in
/// `core/`. Returns [`RetryError::InvalidConfig`] when a policy field is out
/// of range.
pub trait Validator {
    /// Validate the supplied retry policy.
    fn validate(&self, req: ValidationRequest) -> Result<(), RetryError>;
}
