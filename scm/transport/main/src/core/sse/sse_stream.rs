//! `impl SseStream` — the declaration lives in `api/`.

use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;

use crate::api::{HttpEgressError, SseEvent, SseStream};

impl SseStream {
    /// Wrap an already-constructed event stream.
    pub fn new(
        stream: impl Stream<Item = Result<SseEvent, HttpEgressError>> + Send + 'static,
    ) -> Self {
        Self(Box::pin(stream))
    }
}

impl Stream for SseStream {
    type Item = Result<SseEvent, HttpEgressError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}
