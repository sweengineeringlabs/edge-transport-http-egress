//! `impl WsSender` — the declaration lives in `api/`.

use tokio::sync::mpsc;

use crate::api::{WsMessage, WsSender};

impl WsSender {
    /// Wrap an already-constructed `mpsc` sender.
    pub fn new(inner: mpsc::UnboundedSender<WsMessage>) -> Self {
        Self(inner)
    }

    /// Push a frame to the remote peer. Returns the frame back on `Err` if
    /// the receiving half has already been dropped.
    pub fn send(&self, msg: WsMessage) -> Result<(), WsMessage> {
        self.0.send(msg).map_err(|e| e.0)
    }
}
