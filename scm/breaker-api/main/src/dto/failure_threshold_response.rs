//! Response for [`crate::BreakerMetrics::failure_threshold`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::BreakerMetrics::failure_threshold`] — the
/// configured failure threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureThresholdResponse {
    /// The configured failure threshold.
    pub value: u32,
}
