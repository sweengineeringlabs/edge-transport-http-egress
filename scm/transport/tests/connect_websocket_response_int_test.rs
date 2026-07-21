//! Integration tests for `ConnectWebsocketResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{
    ConnectWebsocketRequest, HttpEgressError, HttpTransportSvc,
};

/// Without the `websocket` feature flag (this crate's default test build),
/// `connect_websocket` never yields a `ConnectWebsocketResponse` — it must
/// fail with the documented feature-gate error, not silently return an
/// empty/placeholder channel.
#[tokio::test]
async fn test_connect_websocket_response_without_feature_returns_feature_gate_error_happy() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let result = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "ws://127.0.0.1:1/ws".to_string(),
        })
        .await;
    let err = match result {
        Ok(_) => panic!("connect_websocket must fail without the websocket feature"),
        Err(e) => e,
    };

    match err {
        HttpEgressError::Internal(msg) => {
            assert!(
                msg.contains("websocket"),
                "error must name the missing feature, got: {msg}"
            );
        }
        other => panic!("expected Internal feature-gate error, got: {other:?}"),
    }
}

/// The feature-gate error must be returned consistently across repeated
/// calls — not just the first one.
#[tokio::test]
async fn test_connect_websocket_response_repeated_calls_both_fail_edge() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let a = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "ws://127.0.0.1:1/a".to_string(),
        })
        .await;
    let b = stream
        .connect_websocket(ConnectWebsocketRequest {
            url: "ws://127.0.0.1:1/b".to_string(),
        })
        .await;
    assert!(a.is_err() && b.is_err());
}
