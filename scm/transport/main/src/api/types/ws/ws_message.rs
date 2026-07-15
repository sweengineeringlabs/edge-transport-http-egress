//! WebSocket message value object (egress).

use bytes::Bytes;

/// A single WebSocket message frame exchanged with a remote service.
///
/// Use [`WsMessage::text`] for UTF-8 text frames and [`WsMessage::binary`]
/// for binary frames. The `binary` flag drives the WebSocket opcode: `true`
/// → binary frame (opcode 0x2), `false` → text frame (opcode 0x1).
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_transport::WsMessage;
///
/// let text_frame = WsMessage::text("hello world");
/// assert!(!text_frame.binary);
/// assert_eq!(text_frame.data.as_ref(), b"hello world");
///
/// let binary_frame = WsMessage::binary(vec![0x00, 0xFF, 0xAB]);
/// assert!(binary_frame.binary);
/// assert_eq!(binary_frame.data.len(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct WsMessage {
    /// Raw payload bytes.
    pub data: Bytes,
    /// `true` for binary frames; `false` for UTF-8 text frames.
    pub binary: bool,
}

impl WsMessage {
    /// Construct a text frame from a UTF-8 string.
    pub fn text(data: impl Into<String>) -> Self {
        Self {
            data: Bytes::from(data.into().into_bytes()),
            binary: false,
        }
    }

    /// Construct a binary frame.
    pub fn binary(data: impl Into<Bytes>) -> Self {
        Self {
            data: data.into(),
            binary: true,
        }
    }
}
