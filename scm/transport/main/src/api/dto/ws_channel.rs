//! WebSocket full-duplex channel value object (egress).

use crate::api::dto::ws_receiver::WsReceiver;
use crate::api::dto::ws_sender::WsSender;

/// A full-duplex WebSocket channel to a remote service.
///
/// Returned by [`HttpStream::connect_websocket`](crate::HttpStream::connect_websocket) after the handshake.
/// Use [`sender`](WsChannel::sender) to push frames and [`receiver`](WsChannel::receiver) to consume them.
pub struct WsChannel {
    /// Send frames to the remote WebSocket peer.
    pub sender: WsSender,
    /// Receive frames from the remote WebSocket peer.
    pub receiver: WsReceiver,
}
