//! Integration tests for `WsChannel`.

use futures::stream;
use tokio::sync::mpsc;

use edge_transport_http_egress_transport::WsChannel;

#[test]
fn test_ws_channel_struct_can_be_constructed() {
    let (tx, _rx) = mpsc::unbounded_channel();
    let _ch = WsChannel {
        sender: tx,
        receiver: Box::pin(stream::empty()),
    };
}
