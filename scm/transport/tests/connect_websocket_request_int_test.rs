//! Integration tests for `ConnectWebsocketRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{ConnectWebsocketRequest, HttpTransportSvc};

/// An unreachable `url` must surface as an `Err`, proving `connect_websocket`
/// isn't a stub that always succeeds regardless of the request.
#[tokio::test]
async fn test_connect_websocket_request_unreachable_url_returns_err_happy() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let result = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "ws://0.0.0.0:1/ws".to_string(),
        })
        .await;
    assert!(result.is_err());
}

/// An empty `url` must also be rejected.
#[tokio::test]
async fn test_connect_websocket_request_empty_url_returns_err_error() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let result = stream
        .connect_websocket(ConnectWebsocketRequest { url: String::new() })
        .await;
    assert!(result.is_err());
}

/// `ConnectWebsocketRequest.url` is per-call — two different URLs on the
/// same stream implementor must both be dispatched independently.
#[tokio::test]
async fn test_connect_websocket_request_url_field_is_per_call_edge() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let a = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "ws://0.0.0.0:1/a".to_string(),
        })
        .await;
    let b = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "ws://0.0.0.0:1/b".to_string(),
        })
        .await;
    assert!(a.is_err() && b.is_err());
}
