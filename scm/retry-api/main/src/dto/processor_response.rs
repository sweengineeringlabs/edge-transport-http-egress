//! Response DTO for [`crate::Processor::describe`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::Processor::describe`] — a human-readable
/// identifier for a processor unit, for log / trace output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorResponse {
    /// Canonical identifier for this processor unit.
    pub label: String,
}
