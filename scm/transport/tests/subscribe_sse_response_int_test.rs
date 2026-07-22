//! Integration tests for `SubscribeSseResponse`.
//!
//! `HttpTransportSvc::default_http_stream_outbound` (the only public factory
//! that returns `Box<dyn HttpStream>`) wires the `cassette` layer in strict
//! `"replay"` mode with no public override, so a live 2xx round trip cannot
//! be driven through the public API without a pre-baked, port-matched
//! fixture (which would reintroduce the fixed-port flakiness this suite
//! deliberately avoids — every other test binds `127.0.0.1:0`). These tests
//! instead prove `subscribe_sse` never fabricates a `SubscribeSseResponse`
//! on failure: it surfaces the real underlying error
//! (`HttpEgressError::ConnectionFailed`, wrapping the cassette-miss reason)
//! rather than swallowing it or returning an empty/placeholder stream.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{
    HttpEgressError, HttpTransportSvc, SubscribeSseRequest,
};

/// A subscription that cannot be satisfied (no matching cassette recording,
/// and no live network to fall back to) must return the specific
/// `ConnectionFailed` variant, not a generic/placeholder response.
#[tokio::test]
async fn test_subscribe_sse_response_unsatisfied_call_returns_connection_failed_happy() {
    let stream_outbound = HttpTransportSvc::default_http_stream_outbound().expect("ok");
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
    let stream_outbound = HttpTransportSvc::default_http_stream_outbound().expect("ok");
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
