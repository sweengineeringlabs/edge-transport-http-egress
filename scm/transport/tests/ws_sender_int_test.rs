//! Integration tests for `WsSender`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use tokio::sync::mpsc;

use edge_transport_http_egress_transport::{WsMessage, WsSender};

/// @covers: WsSender
#[tokio::test]
async fn test_ws_sender_type_can_be_constructed_from_mpsc_channel() {
    let (tx, mut rx) = mpsc::unbounded_channel::<WsMessage>();
    let sender = WsSender::new(tx);
    sender
        .send(WsMessage::binary(vec![0x00, 0xFF]))
        .expect("send over WsSender must succeed");
    let received = rx
        .recv()
        .await
        .expect("a frame must arrive on the receiver");
    assert!(received.binary, "a binary frame must be flagged binary");
    assert_eq!(
        received.data.as_slice(),
        &[0x00, 0xFF],
        "WsSender must carry the frame payload verbatim"
    );
}
