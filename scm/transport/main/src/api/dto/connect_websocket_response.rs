//! Response for [`crate::api::HttpStream::connect_websocket`].

use crate::api::dto::ws_channel::WsChannel;

/// Output of [`crate::api::HttpStream::connect_websocket`] — the full-duplex
/// WebSocket channel. Not serializable (wraps live sender/receiver handles),
/// so this DTO intentionally has no `Serialize`/`Deserialize` derive.
pub struct ConnectWebsocketResponse {
    /// The connected WebSocket channel.
    pub channel: WsChannel,
}
