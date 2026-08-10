//! Integration tests for `SubscribeSseResponse`.
//!
//! `HttpTransportSvc::plain_http_stream` returns a bare `Box<dyn HttpStream>`
//! (no middleware layers). These tests drive `subscribe_sse` against
//! unreachable addresses and prove it never fabricates a `SubscribeSseResponse`
//! on failure: it surfaces the real underlying error
//! (`HttpEgressError::ConnectionFailed`) rather than swallowing it or returning
//! an empty/placeholder stream.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{
    HttpConfig, HttpEgressError, HttpTransportSvc, SubscribeSseRequest,
};

/// A subscription that cannot be satisfied (no live network to reach) must
/// return the specific `ConnectionFailed` variant, not a generic/placeholder
/// response.
#[tokio::test]
async fn test_subscribe_sse_response_unsatisfied_call_returns_connection_failed_happy() {
    let stream_outbound = HttpTransportSvc::plain_http_stream(HttpConfig::default()).expect("ok");
    let result = stream_outbound
        .subscribe_sse(SubscribeSseRequest {
            url: "http://127.0.0.1:9/events".to_string(),
        })
        .await;

    match result {
        Err(HttpEgressError::ConnectionFailed(_)) => {}
        Err(other) => panic!("expected ConnectionFailed, got: {other:?}"),
        Ok(_) => panic!("expected ConnectionFailed, got Ok"),
    }
}

/// Two independent, distinctly-URLed subscriptions must each fail on their
/// own with the same error shape — a shared/cached failure state would mask
/// the second call's real outcome.
#[tokio::test]
async fn test_subscribe_sse_response_independent_calls_both_fail_edge() {
    let stream_outbound = HttpTransportSvc::plain_http_stream(HttpConfig::default()).expect("ok");
    let a = stream_outbound
        .subscribe_sse(SubscribeSseRequest {
            url: "http://0.0.0.0:1/a".to_string(),
        })
        .await;
    let b = stream_outbound
        .subscribe_sse(SubscribeSseRequest {
            url: "http://0.0.0.0:1/b".to_string(),
        })
        .await;
    assert!(matches!(a, Err(HttpEgressError::ConnectionFailed(_))));
    assert!(matches!(b, Err(HttpEgressError::ConnectionFailed(_))));
}
