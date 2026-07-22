//! Request for [`crate::api::RateMetrics::rate_limit`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::RateMetrics::rate_limit`] — no parameters are
/// needed; the struct exists to satisfy the uniform request/response
/// contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RateLimitRequest;
