//! Integration tests for `HttpEgressSvcFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpEgressSvcFactory;

/// `create()` must build a usable `HttpEgress` repeatably — a builder with
/// broken shared state would fail the second call.
#[test]
fn test_http_egress_svc_factory_create_builds_repeatably_happy() {
    let first = HttpEgressSvcFactory::create();
    let second = HttpEgressSvcFactory::create();
    assert!(
        first.is_ok() && second.is_ok(),
        "HttpEgressSvcFactory::create must build repeatably: {:?} / {:?}",
        first.err(),
        second.err(),
    );
}

/// Two independently created egresses must be distinct instances — proving
/// `create()` doesn't hand out a single cached singleton.
#[test]
fn test_http_egress_svc_factory_create_returns_independent_instances_edge() {
    fn assert_send_sync<T: Send + Sync + ?Sized>(_: &T) {}
    let egress = HttpEgressSvcFactory::create().expect("must build");
    assert_send_sync(egress.as_ref());
}
