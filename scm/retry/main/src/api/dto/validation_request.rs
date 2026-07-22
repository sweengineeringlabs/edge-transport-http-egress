//! Request DTO for [`crate::api::Validator::validate`].

use serde::{Deserialize, Serialize};

use crate::api::RetryConfig;

/// Input to [`crate::api::Validator::validate`] — the policy to check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRequest {
    /// The retry policy to validate.
    pub config: RetryConfig,
}
