//! Integration tests for the MetricsHttpEgress SAF wrapper.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_transport_http_egress_transport_api::HttpMetricsObserver;
use edge_transport_http_egress_transport_api::{HttpConfig, TransportFactory};
use edge_transport_http_egress_transport_std::StdTransportFactory;

/// @covers: observe_http_egress
#[test]
fn test_observe_http_egress_wraps_plain_outbound_without_error() {
    let inner = StdTransportFactory
        .plain_http_egress(HttpConfig::default())
        .expect("plain_http_egress");
    let provider: HttpMetricsObserver = Arc::new(|_| {});
    let _observed = StdTransportFactory.observe_http_egress(inner, provider);
}

/// @covers: observe_http_egress
#[test]
fn test_observe_http_egress_two_instances_share_provider() {
    let inner_a = StdTransportFactory
        .plain_http_egress(HttpConfig::default())
        .expect("first plain_http_egress");
    let inner_b = StdTransportFactory
        .plain_http_egress(HttpConfig::default())
        .expect("second plain_http_egress");
    let provider: HttpMetricsObserver = Arc::new(|_| {});
    let _a = StdTransportFactory.observe_http_egress(inner_a, Arc::clone(&provider));
    let _b = StdTransportFactory.observe_http_egress(inner_b, provider);
}
