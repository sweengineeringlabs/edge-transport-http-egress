//! Response DTO for [`crate::api::Processor::describe`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::api::Processor::describe`] — a human-readable
/// identifier for a processor unit, for log / trace output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorResponse {
    /// Canonical identifier for this processor unit.
    pub label: String,
}
