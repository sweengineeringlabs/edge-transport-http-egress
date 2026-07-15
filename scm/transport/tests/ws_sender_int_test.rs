//! Integration tests for `WsSender`.

use tokio::sync::mpsc;

use edge_transport_http_egress_transport::{WsMessage, WsSender};

#[test]
fn test_ws_sender_type_can_be_constructed_from_mpsc_channel() {
    let (tx, _rx) = mpsc::unbounded_channel::<WsMessage>();
    let _: WsSender = tx;
}
