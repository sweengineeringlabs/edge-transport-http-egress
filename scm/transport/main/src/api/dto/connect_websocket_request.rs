//! Request for [`crate::api::HttpStream::connect_websocket`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::HttpStream::connect_websocket`] — the WebSocket URL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectWebsocketRequest {
    /// The URL of the WebSocket server to connect to.
    pub url: String,
}
