//! `BreakerMetrics` — circuit breaker state inspection contract.

use crate::api::{BreakerError, FailureThresholdRequest, FailureThresholdResponse};

/// Circuit-breaker state access.
pub trait BreakerMetrics: Send + Sync {
    /// Return the failure threshold configured for this breaker.
    fn failure_threshold(
        &self,
        request: FailureThresholdRequest,
    ) -> Result<FailureThresholdResponse, BreakerError>;
}
