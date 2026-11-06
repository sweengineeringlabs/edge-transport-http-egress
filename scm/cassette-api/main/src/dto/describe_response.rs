//! Response for [`crate::Processor::describe`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::Processor::describe`] — identifies the processor
/// in log/trace output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeResponse {
    /// The processor's identifying label.
    pub value: String,
}
