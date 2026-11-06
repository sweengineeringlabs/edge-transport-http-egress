//! Response for [`crate::HttpStream::connect_websocket`].

use crate::WsChannel;

/// Output of [`crate::HttpStream::connect_websocket`] — the full-duplex
/// WebSocket channel. Not serializable (wraps live sender/receiver handles),
/// so this DTO intentionally has no `Serialize`/`Deserialize` derive.
pub struct ConnectWebsocketResponse {
    /// The connected WebSocket channel.
    pub channel: WsChannel,
}
