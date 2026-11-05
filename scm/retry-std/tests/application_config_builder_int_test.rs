//! Integration tests for `new_app_config_builder` in `edge_transport_http_egress_retry`.

use edge_transport_http_egress_retry_api::RetryFactory;
use edge_transport_http_egress_retry_std::StdRetryFactory;

/// @covers: new_app_config_builder
#[test]
fn std_struct_svc_create_config_builder_returns_seeded_builder_int_test() {
    let builder = StdRetryFactory.new_app_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must be seeded with crate name"
    );
    assert!(
        !builder.version().is_empty(),
        "builder must be seeded with crate version"
    );
}
