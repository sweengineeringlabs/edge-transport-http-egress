//! Integration tests for `WsChannel`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use futures::stream;
use tokio::sync::mpsc;

use edge_transport_http_egress_transport::{WsChannel, WsMessage, WsReceiver, WsSender};

/// @covers: WsChannel
#[tokio::test]
async fn test_ws_channel_struct_can_be_constructed() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let ch = WsChannel {
        sender: WsSender::new(tx),
        receiver: WsReceiver::new(stream::empty()),
    };
    // The channel's sender must deliver a real frame to the paired receiver.
    ch.sender
        .send(WsMessage::text("ping"))
        .expect("send over WsChannel sender must succeed");
    let received = rx.recv().await.expect("a frame must arrive");
    assert_eq!(
        received.data.as_slice(),
        b"ping",
        "WsChannel sender must carry the frame payload verbatim"
    );
    assert!(!received.binary, "a text frame must not be flagged binary");
}
