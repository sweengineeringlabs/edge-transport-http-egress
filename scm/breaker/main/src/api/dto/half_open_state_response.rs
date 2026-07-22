//! Response for [`crate::api::HostBreaker::is_half_open`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::api::HostBreaker::is_half_open`] — `true` when the
/// breaker is in the HalfOpen (probe) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HalfOpenStateResponse {
    /// `true` when the breaker is HalfOpen.
    pub value: bool,
}
