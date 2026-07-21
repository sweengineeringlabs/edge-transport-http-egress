//! `CircuitBreakerNode` — per-host state machine contract.

use crate::api::{AdmitRequest, AdmitResponse, BreakerError, RecordRequest, RecordResponse};

/// Contract for per-host circuit breaker state machines.
pub trait CircuitBreakerNode {
    /// Decide whether to admit a new request based on the current breaker state.
    fn admit(&mut self, request: AdmitRequest) -> Result<AdmitResponse, BreakerError>;

    /// Record the outcome of a request to update the breaker state.
    fn record(&mut self, request: RecordRequest) -> Result<RecordResponse, BreakerError>;
}
