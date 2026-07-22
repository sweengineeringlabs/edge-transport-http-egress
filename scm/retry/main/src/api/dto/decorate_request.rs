//! Request DTO for [`crate::api::Processor::decorate`].

use serde::{Deserialize, Serialize};

use crate::api::RetryConfig;

/// Input to [`crate::api::Processor::decorate`] — the policy the built
/// middleware layer will enforce.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorateRequest {
    /// The retry policy the built layer enforces.
    pub config: RetryConfig,
}
