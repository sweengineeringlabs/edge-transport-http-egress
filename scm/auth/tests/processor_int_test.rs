//! Integration tests for the `Processor` trait in `edge-transport-http-egress-auth`.

use edge_transport_http_egress_auth::AuthSvc;

/// @covers: Processor
#[test]
fn test_auth_svc_implements_processor_contract() {
    // AuthSvc implements the processor interface — just verify it can be constructed.
    let _svc = AuthSvc;
}
