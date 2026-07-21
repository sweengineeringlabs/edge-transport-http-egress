//! Response for [`crate::api::HostBreaker::is_open`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::api::HostBreaker::is_open`] — `true` when the
/// breaker is in the Open (fail-fast) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenStateResponse {
    /// `true` when the breaker is Open.
    pub value: bool,
}
