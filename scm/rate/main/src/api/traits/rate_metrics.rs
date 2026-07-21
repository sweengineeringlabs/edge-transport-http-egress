//! `RateMetrics` — rate-limiter policy inspection contract.

use crate::api::{RateError, RateLimitRequest, RateLimitResponse};

/// Rate-limiter policy access.
///
/// Provides observable inspection of a rate layer's configured limit that
/// consumers can rely on without depending on the concrete layer internals.
pub trait RateMetrics: Send + Sync {
    /// Return the configured sustained refill rate for this layer.
    fn rate_limit(&self, request: RateLimitRequest) -> Result<RateLimitResponse, RateError>;
}
