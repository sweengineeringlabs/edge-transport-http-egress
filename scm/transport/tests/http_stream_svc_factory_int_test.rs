//! Integration tests for `HttpStreamSvcFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpStreamSvcFactory;

/// `create()` must build a usable `HttpStream` repeatably.
#[test]
fn test_http_stream_svc_factory_create_builds_repeatably_happy() {
    let first = HttpStreamSvcFactory::create();
    let second = HttpStreamSvcFactory::create();
    assert!(
        first.is_ok() && second.is_ok(),
        "HttpStreamSvcFactory::create must build repeatably: {:?} / {:?}",
        first.err(),
        second.err(),
    );
}

/// The returned stream implementor must be usable as a boxed trait object
/// across thread boundaries.
#[test]
fn test_http_stream_svc_factory_create_returns_send_sync_edge() {
    fn assert_send_sync<T: Send + Sync + ?Sized>(_: &T) {}
    let stream = HttpStreamSvcFactory::create().expect("must build");
    assert_send_sync(stream.as_ref());
}
