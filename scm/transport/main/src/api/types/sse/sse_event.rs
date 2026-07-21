//! SSE event value object (egress).

/// A single Server-Sent Event frame received from a remote SSE feed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SseEvent {
    /// Optional `event:` field — names the event type.
    pub event: Option<String>,
    /// The `data:` field — the payload carried by the event.
    pub data: String,
    /// Optional `id:` field — the last-event-ID for reconnect resumption.
    pub id: Option<String>,
}
