//! Response for [`crate::api::HttpStream::subscribe_sse`].

use crate::api::dto::sse_stream::SseStream;

/// Output of [`crate::api::HttpStream::subscribe_sse`] — the lazy SSE event
/// stream. Not serializable (wraps a live, boxed `Stream`), so this DTO
/// intentionally has no `Serialize`/`Deserialize` derive.
pub struct SubscribeSseResponse {
    /// The subscribed SSE event stream.
    pub stream: SseStream,
}
