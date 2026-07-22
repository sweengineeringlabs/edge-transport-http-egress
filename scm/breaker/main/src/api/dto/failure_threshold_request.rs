//! Request for [`crate::api::BreakerMetrics::failure_threshold`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::BreakerMetrics::failure_threshold`] — no
/// parameters are needed; the struct exists to satisfy the uniform
/// request/response contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureThresholdRequest;
