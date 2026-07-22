//! Interface counterpart for the corresponding core/ implementation.

use crate::api::{
    BreakerError, ClosedStateRequest, ClosedStateResponse, HalfOpenStateRequest,
    HalfOpenStateResponse, OpenStateRequest, OpenStateResponse,
};

/// Trait for per-host breaker state machines.
///
/// Provides observable state inspection methods that consumers can rely on
/// without depending on the concrete `core::host::breaker::HostBreaker` type.
#[cfg_attr(not(feature = "loadbalancer"), allow(dead_code))]
pub trait HostBreaker: Send + Sync {
    /// Returns whether the breaker is in the Open (fail-fast) state.
    fn is_open(&self, request: OpenStateRequest) -> Result<OpenStateResponse, BreakerError>;

    /// Returns whether the breaker is in the HalfOpen (probe) state.
    fn is_half_open(
        &self,
        request: HalfOpenStateRequest,
    ) -> Result<HalfOpenStateResponse, BreakerError>;

    /// Returns whether the breaker is in the Closed (normal) state.
    fn is_closed(&self, request: ClosedStateRequest) -> Result<ClosedStateResponse, BreakerError>;
}
