//! Integration tests for `HttpEgressObject` (`dyn HttpEgress`).
//!
//! `HttpEgressObject` is the public dyn-safe alias for the `HttpEgress` trait.
//! These tests build a real egress, coerce it to the alias, and dispatch real
//! calls through it — an unreachable host must surface as an error.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{
    GetRequest, HttpConfig, HttpEgressObject, HttpRequest, HttpTransportSvc,
};

/// @covers: HttpEgressObject
#[tokio::test]
async fn transport_struct_http_egress_object_is_object_safe_int_test() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("build egress");
    let obj: &HttpEgressObject = egress.as_ref();
    let result = obj
        .send(HttpRequest::get("http://0.0.0.0:1/probe".to_string()))
        .await;
    assert!(
        result.is_err(),
        "dispatch through HttpEgressObject must reach send() and fail on an unreachable host"
    );
}

/// @covers: HttpEgressObject alias accessibility
#[tokio::test]
async fn transport_struct_http_egress_object_alias_is_accessible_int_test() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("build egress");
    let obj: &HttpEgressObject = egress.as_ref();
    let sent = obj
        .send(HttpRequest::get("http://0.0.0.0:1/a".to_string()))
        .await;
    let got = obj
        .get(GetRequest {
            url: "http://0.0.0.0:1/b".to_string(),
        })
        .await;
    assert!(
        sent.is_err() && got.is_err(),
        "both calls dispatched through the alias must fail on an unreachable host"
    );
}
