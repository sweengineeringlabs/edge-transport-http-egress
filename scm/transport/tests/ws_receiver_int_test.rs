//! Integration tests for `WsReceiver`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{WsMessage, WsReceiver};
use futures::stream;
use futures::StreamExt as _;

/// @covers: WsReceiver
#[tokio::test]
async fn test_ws_receiver_type_empty_stream_is_valid() {
    let mut r = WsReceiver::new(stream::empty());
    assert!(
        r.next().await.is_none(),
        "an empty WsReceiver must yield no frames"
    );
}

/// @covers: WsReceiver
#[tokio::test]
async fn test_ws_receiver_type_yields_pushed_frames_edge() {
    let r = WsReceiver::new(stream::iter(vec![
        Ok(WsMessage::text("a")),
        Ok(WsMessage::binary(vec![1u8, 2, 3])),
    ]));
    let frames: Vec<_> = r.collect().await;
    assert_eq!(frames.len(), 2, "receiver must yield both pushed frames");
    assert!(
        frames[1].as_ref().expect("second frame is Ok").binary,
        "the second frame must be flagged binary"
    );
}
