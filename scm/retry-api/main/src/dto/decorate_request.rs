//! Request DTO for [`crate::Processor::decorate`].

use serde::{Deserialize, Serialize};

use crate::RetryConfig;

/// Input to [`crate::Processor::decorate`] — the policy the built
/// middleware layer will enforce.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorateRequest {
    /// The retry policy the built layer enforces.
    pub config: RetryConfig,
}
