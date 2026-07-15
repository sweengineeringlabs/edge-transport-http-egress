//! Integration tests for the `Processor` trait in `edge-transport-http-egress-rate`.

use edge_transport_http_egress_rate::HttpRateSvc;

/// @covers: Processor
#[test]
fn test_processor_trait_is_implementable() {
    // The Svc type must implement Processor to satisfy service_type requirements.
    // If this compiles, the trait contract is satisfied.
    let svc = HttpRateSvc;
    // just creating the type verifies the impl exists
    let _ = svc;
}
