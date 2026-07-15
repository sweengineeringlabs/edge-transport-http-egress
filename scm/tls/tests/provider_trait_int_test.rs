//! Integration tests for `describe_tls_provider` — provider accessibility from crate root.

use edge_transport_http_egress_tls::{describe_tls_provider, HttpTlsSvc};

/// @covers: describe_tls_provider
/// Proves the SAF wrapper for `Provider::describe` is accessible from the crate root
/// and delegates correctly to the underlying implementation.
#[test]
fn tls_trait_provider_describe_is_accessible_from_crate_root_int_test() {
    let svc = HttpTlsSvc;
    assert_eq!(describe_tls_provider(&svc), "http-tls");
}
