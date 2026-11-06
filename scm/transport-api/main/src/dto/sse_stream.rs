//! SSE stream type (egress).

use std::pin::Pin;

use futures::Stream;

use crate::HttpEgressError;
use crate::SseEvent;

/// A lazy stream of [`SseEvent`] items consumed from a remote SSE feed.
///
/// The outbound implementation decodes `text/event-stream` frames from the
/// HTTP response body and emits them as [`SseEvent`] items. Implements
/// [`Stream`] — drive it with [`futures::StreamExt::next`].
pub struct SseStream(
    pub(crate) Pin<Box<dyn Stream<Item = Result<SseEvent, HttpEgressError>> + Send>>,
);
