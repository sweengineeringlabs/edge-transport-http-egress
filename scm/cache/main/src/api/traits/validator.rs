//! `Validator` — configuration validation contract for the cache crate.

use crate::api::{CacheError, ConfigValidationRequest};

/// Configuration validation contract.
pub trait Validator: Send + Sync {
    /// Validate the configuration, returning `Ok(())` when valid or a
    /// [`CacheError::InvalidConfig`] describing the offending field.
    fn validate(&self, request: ConfigValidationRequest) -> Result<(), CacheError>;
}
