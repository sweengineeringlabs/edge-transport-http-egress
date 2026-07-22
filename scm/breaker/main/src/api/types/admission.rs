//! `Admission` — decision returned when a request arrives at the circuit breaker.

use serde::{Deserialize, Serialize};

/// Decision returned when a new request arrives at the circuit breaker.
///
/// The breaker evaluates `Admission` before dispatching the request. If `RejectOpen`,
/// the call never reaches the upstream and `HttpEgressError::ServiceUnavailable` is
/// returned immediately to the caller.
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_breaker::Admission;
///
/// assert_eq!(Admission::Proceed, Admission::Proceed);
/// assert_ne!(Admission::Proceed, Admission::RejectOpen);
///
/// let decision = Admission::Proceed;
/// match decision {
///     Admission::Proceed    => {} // dispatch the request
///     Admission::RejectOpen => {} // return ServiceUnavailable immediately
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Admission {
    /// Pass through — record the outcome afterward.
    Proceed,
    /// Breaker is open — fail fast without calling upstream.
    RejectOpen,
}
