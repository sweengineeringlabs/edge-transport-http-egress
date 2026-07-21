//! `impl HttpByteStream` — the declaration lives in `api/`.

use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;

use crate::api::{HttpByteStream, HttpEgressError};

impl HttpByteStream {
    /// Wrap an already-constructed byte-chunk stream.
    pub fn new(
        stream: impl Stream<Item = Result<Vec<u8>, HttpEgressError>> + Send + 'static,
    ) -> Self {
        Self(Box::pin(stream))
    }
}

impl Stream for HttpByteStream {
    type Item = Result<Vec<u8>, HttpEgressError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}
