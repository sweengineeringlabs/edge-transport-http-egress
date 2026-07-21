//! Request DTO for [`crate::api::Processor::describe`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::Processor::describe`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorRequest {
    /// When `true`, the description includes the resolved policy detail.
    pub verbose: bool,
}
