//! `Validator` — config validation contract.

use crate::api::{ConfigValidationRequest, LoadbalancerMiddlewareError};

/// Validates that a [`LoadbalancerConfig`](crate::api::LoadbalancerConfig) is
/// well-formed before the middleware is constructed.
pub trait Validator: Send + Sync {
    /// Return `Ok(())` if the configuration is valid, or an
    /// [`LoadbalancerMiddlewareError::InvalidConfig`] describing the first
    /// violation found.
    fn validate(&self, request: ConfigValidationRequest)
        -> Result<(), LoadbalancerMiddlewareError>;
}
