//! Error type for [`crate::Validator::validate`].

use thiserror::Error;

/// Errors raised by [`Validator::validate`](crate::Validator::validate).
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    /// The value being validated failed a structural check.
    #[error("invalid: {0}")]
    Invalid(String),
}
