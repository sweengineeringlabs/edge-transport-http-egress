//! WebSocket send-side channel type (egress).

use tokio::sync::mpsc;

use crate::api::types::ws::WsMessage;

/// The send half of a [`WsChannel`](super::ws_channel::WsChannel) (egress).
///
/// Push [`WsMessage`] frames to the remote WebSocket peer via [`send`](WsSender::send).
pub struct WsSender(pub(crate) mpsc::UnboundedSender<WsMessage>);
