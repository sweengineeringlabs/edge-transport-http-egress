//! WebSocket message value object (egress).

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
/// assert_eq!(text_frame.data.as_slice(), b"hello world");
///
/// let binary_frame = WsMessage::binary(vec![0x00, 0xFF, 0xAB]);
/// assert!(binary_frame.binary);
/// assert_eq!(binary_frame.data.len(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct WsMessage {
    /// Raw payload bytes.
    pub data: Vec<u8>,
    /// `true` for binary frames; `false` for UTF-8 text frames.
    pub binary: bool,
}
