//! Integration tests for `WsReceiver`.

use edge_transport_http_egress_transport::WsReceiver;
use futures::stream;

#[test]
fn test_ws_receiver_type_empty_stream_is_valid() {
    let _r: WsReceiver = Box::pin(stream::empty());
}
