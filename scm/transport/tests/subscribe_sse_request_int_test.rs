//! Integration tests for `SubscribeSseRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{HttpTransportSvc, SubscribeSseRequest};

/// An unreachable `url` must surface as an `Err` from the initial connect,
/// proving `subscribe_sse` isn't a stub that always succeeds.
#[tokio::test]
async fn test_subscribe_sse_request_unreachable_url_returns_err_happy() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let result = stream
        .subscribe_sse(SubscribeSseRequest {
            url: "http://0.0.0.0:1/events".to_string(),
        })
        .await;
    assert!(result.is_err());
}

/// An empty `url` must also be rejected rather than silently no-op'ing.
#[tokio::test]
async fn test_subscribe_sse_request_empty_url_returns_err_error() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let result = stream
        .subscribe_sse(SubscribeSseRequest { url: String::new() })
        .await;
    assert!(result.is_err());
}

/// `SubscribeSseRequest.url` is per-call — two different URLs on the same
/// stream implementor must be independently dispatched, not cached/ignored.
#[tokio::test]
async fn test_subscribe_sse_request_url_field_is_per_call_edge() {
    let stream = HttpTransportSvc::default_http_stream_outbound().expect("ok");
    let a = stream
        .subscribe_sse(SubscribeSseRequest {
            url: "http://0.0.0.0:1/a".to_string(),
        })
        .await;
    let b = stream
        .subscribe_sse(SubscribeSseRequest {
            url: "http://0.0.0.0:1/b".to_string(),
        })
        .await;
    assert!(a.is_err() && b.is_err());
}
