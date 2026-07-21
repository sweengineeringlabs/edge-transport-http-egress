//! Response for [`crate::api::HostBreaker::is_closed`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::api::HostBreaker::is_closed`] — `true` when the
/// breaker is in the Closed (normal) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClosedStateResponse {
    /// `true` when the breaker is Closed.
    pub value: bool,
}
