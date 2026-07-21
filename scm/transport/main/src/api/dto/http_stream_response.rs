//! Streaming HTTP response type.

use std::collections::HashMap;

use crate::api::types::http::HttpByteStream;

/// A streaming HTTP response — status and headers are available immediately;
/// the body arrives as a lazy [`HttpByteStream`] of byte chunks.
///
/// Unlike [`HttpResponse`](super::HttpResponse), the body is **not buffered**.
/// Callers drive the stream with [`futures::StreamExt::next`]; the connection
/// stays open until the stream is exhausted or dropped.
///
/// # Retry caveat
///
/// Retry middleware applies to the *connection* only. A partially-consumed
/// stream cannot be rewound and retried transparently. If the stream drops
/// mid-response, the caller is responsible for reconnecting.
pub struct HttpStreamResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response headers (lowercase keys).
    pub headers: HashMap<String, String>,
    /// Lazy byte stream. Drive with `futures::StreamExt::next`.
    pub body: HttpByteStream,
}
