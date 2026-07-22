//! `Outcome` — result of a dispatched request as seen by the breaker.

use serde::{Deserialize, Serialize};

/// Outcome of a dispatched request.
///
/// The breaker records this after each request that was admitted. `Failure`
/// increments the failure counter; `Success` in `HalfOpen` state increments
/// the recovery counter toward `reset_after_successes`.
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_breaker::Outcome;
///
/// assert_eq!(Outcome::Success, Outcome::Success);
/// assert_ne!(Outcome::Success, Outcome::Failure);
///
/// let outcome = Outcome::Failure;
/// match outcome {
///     Outcome::Success => {} // decrement failure counter or count toward reset
///     Outcome::Failure => {} // increment failure counter; may trip open
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    /// The request completed successfully.
    Success,
    /// The request failed or returned a configured failure status.
    Failure,
}
