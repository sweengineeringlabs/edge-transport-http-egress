//! `impl WsReceiver` — the declaration lives in `api/`.

use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;

use crate::api::{HttpEgressError, WsMessage, WsReceiver};

impl WsReceiver {
    /// Wrap an already-constructed byte/frame stream.
    pub fn new(
        stream: impl Stream<Item = Result<WsMessage, HttpEgressError>> + Send + 'static,
    ) -> Self {
        Self(Box::pin(stream))
    }
}

impl Stream for WsReceiver {
    type Item = Result<WsMessage, HttpEgressError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}
