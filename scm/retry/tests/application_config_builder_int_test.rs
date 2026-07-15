//! Integration tests for `create_config_builder` in `edge_transport_http_egress_retry`.

use edge_transport_http_egress_retry::HttpRetrySvc;

/// @covers: HttpRetrySvc::create_config_builder — dep coverage for swe-edge-configbuilder
#[test]
fn retry_struct_svc_create_config_builder_returns_seeded_builder_int_test() {
    let builder = HttpRetrySvc::create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must be seeded with crate name"
    );
    assert!(
        !builder.version().is_empty(),
        "builder must be seeded with crate version"
    );
}
