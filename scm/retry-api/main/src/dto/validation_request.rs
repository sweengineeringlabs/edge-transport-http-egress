//! Request DTO for [`crate::Validator::validate`].

use serde::{Deserialize, Serialize};

use crate::RetryConfig;

/// Input to [`crate::Validator::validate`] — the policy to check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRequest {
    /// The retry policy to validate.
    pub config: RetryConfig,
}
