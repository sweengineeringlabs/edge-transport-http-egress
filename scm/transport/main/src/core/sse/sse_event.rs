//! `impl SseEvent` — the declaration lives in `api/`.

use crate::api::SseEvent;

impl SseEvent {
    /// Construct a data-only event with no type or ID.
    pub fn data(data: impl Into<String>) -> Self {
        Self {
            event: None,
            data: data.into(),
            id: None,
        }
    }
}
