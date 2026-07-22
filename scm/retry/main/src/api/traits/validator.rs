//! `Validator` — configuration validation contract.

use crate::api::RetryError;
use crate::api::ValidationRequest;

/// Configuration validation contract.
///
/// Implemented by [`DefaultValidator`](crate::api::RetryConfig) validators in
/// `core/`. Returns [`RetryError::InvalidConfig`] when a policy field is out
/// of range.
pub trait Validator {
    /// Validate the supplied retry policy.
    fn validate(&self, req: ValidationRequest) -> Result<(), RetryError>;
}
