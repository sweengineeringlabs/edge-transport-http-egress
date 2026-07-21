//! Integration tests for `WsMessage`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::WsMessage;

/// @covers: text
#[test]
fn test_ws_message_struct_text_sets_binary_false() {
    let m = WsMessage::text("hi");
    assert!(!m.binary);
    assert_eq!(m.data.as_slice(), b"hi");
}

/// @covers: binary
#[test]
fn test_ws_message_struct_binary_sets_binary_true() {
    let m = WsMessage::binary(vec![0xffu8]);
    assert!(m.binary);
}
