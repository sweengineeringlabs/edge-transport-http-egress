//! A validatable wrapper around HttpConfig.
//!
//! Declaration only. The `Validator` impl lives in `core/`.

use crate::api::types::HttpConfig;

/// Wraps an [`HttpConfig`](crate::HttpConfig) to make it validatable via the `Validator` trait.
pub struct ValidatableHttpConfig {
    /// The HTTP configuration to validate.
    pub config: HttpConfig,
}
