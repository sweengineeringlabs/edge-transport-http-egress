//! Integration tests for `describe_tls_provider` — `Provider` contract via SAF wrapper.

use edge_transport_http_egress_tls::{describe_tls_provider, HttpTlsSvc};

/// @covers: describe_tls_provider
#[test]
fn tls_trait_provider_describe_returns_http_tls_label_int_test() {
    let svc = HttpTlsSvc;
    assert_eq!(describe_tls_provider(&svc), "http-tls");
}
