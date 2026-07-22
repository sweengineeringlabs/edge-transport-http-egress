//! `impl WsMessage` — the declaration lives in `api/`.

use crate::api::WsMessage;

impl WsMessage {
    /// Construct a text frame from a UTF-8 string.
    pub fn text(data: impl Into<String>) -> Self {
        Self {
            data: data.into().into_bytes(),
            binary: false,
        }
    }

    /// Construct a binary frame.
    pub fn binary(data: impl Into<Vec<u8>>) -> Self {
        Self {
            data: data.into(),
            binary: true,
        }
    }
}
