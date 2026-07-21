//! WebSocket receive-side stream type (egress).

use std::pin::Pin;

use futures::Stream;

use crate::api::error::HttpEgressError;
use crate::api::types::ws::WsMessage;

/// The receive half of a [`WsChannel`](super::ws_channel::WsChannel) (egress).
///
/// Yields [`WsMessage`] frames from the remote WebSocket peer until the
/// connection is closed. Implements [`Stream`] — drive it with
/// [`futures::StreamExt::next`].
pub struct WsReceiver(
    pub(crate) Pin<Box<dyn Stream<Item = Result<WsMessage, HttpEgressError>> + Send>>,
);
