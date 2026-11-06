//! Response for [`crate::HttpStream::subscribe_sse`].

use crate::SseStream;

/// Output of [`crate::HttpStream::subscribe_sse`] — the lazy SSE event
/// stream. Not serializable (wraps a live, boxed `Stream`), so this DTO
/// intentionally has no `Serialize`/`Deserialize` derive.
pub struct SubscribeSseResponse {
    /// The subscribed SSE event stream.
    pub stream: SseStream,
}
