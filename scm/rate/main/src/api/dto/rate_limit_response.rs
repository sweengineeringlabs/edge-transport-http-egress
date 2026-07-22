//! Response for [`crate::api::RateMetrics::rate_limit`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::api::RateMetrics::rate_limit`] — the configured
/// sustained token refill rate (tokens per second).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RateLimitResponse {
    /// Sustained refill rate, tokens per second.
    pub tokens_per_second: u32,
}
