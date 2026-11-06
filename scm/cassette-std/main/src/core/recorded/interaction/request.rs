//! Serializable shape of a recorded HTTP request.

use serde::{Deserialize, Serialize};

/// The request half of a recorded HTTP exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RecordedRequest {
    pub(crate) method: String,
    pub(crate) url: String,
    /// Hex-encoded SHA256 of the request body (when
    /// `match_on` includes "body_hash"). `None` when the
    /// request had no body or body_hash isn't in match_on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) body_hash: Option<String>,
}
