//! Integration tests for `MetricsHttpEgress` (`dyn HttpEgress`).
//!
//! `MetricsHttpEgress` is the public dyn-safe alias for the metrics-observation
//! HTTP outbound interface. This test dispatches a real call through the alias.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{
    HttpConfig, HttpRequest, HttpTransportSvc, MetricsHttpEgress,
};

/// @covers: MetricsHttpEgress
#[tokio::test]
async fn test_metrics_http_egress_type_is_object_safe() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("build egress");
    let obj: &MetricsHttpEgress = egress.as_ref();
    let result = obj
        .send(HttpRequest::get("http://0.0.0.0:1/m".to_string()))
        .await;
    assert!(
        result.is_err(),
        "dispatch through the MetricsHttpEgress alias must fail on an unreachable host"
    );
}
