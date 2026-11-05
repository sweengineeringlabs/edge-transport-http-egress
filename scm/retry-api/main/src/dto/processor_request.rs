//! Request DTO for [`crate::Processor::describe`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::Processor::describe`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorRequest {
    /// When `true`, the description includes the resolved policy detail.
    pub verbose: bool,
}
