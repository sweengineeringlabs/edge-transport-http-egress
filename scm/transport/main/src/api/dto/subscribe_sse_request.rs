//! Request for [`crate::api::HttpStream::subscribe_sse`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::HttpStream::subscribe_sse`] — the SSE feed URL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscribeSseRequest {
    /// The URL of the SSE feed to subscribe to.
    pub url: String,
}
