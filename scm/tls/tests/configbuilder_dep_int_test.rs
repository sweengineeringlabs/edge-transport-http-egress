//! Dependency coverage test for `swe-edge-configbuilder`.
//! @covers: swe-edge-configbuilder
//!
//! Rule 95: `swe-edge-configbuilder` is used in `src/` and must have
//! integration coverage with an explicit `use swe_edge_configbuilder::...` import.

use edge_transport_http_egress_tls::HttpTlsSvc;
use swe_edge_configbuilder::ConfigBuilderImpl;

/// @covers: swe-edge-configbuilder
/// Confirms `HttpTlsSvc::create_config_builder` returns a `ConfigBuilderImpl`
/// seeded with the crate name and version.
#[test]
fn tls_struct_svc_create_config_builder_returns_builder_int_test() {
    let builder: ConfigBuilderImpl = HttpTlsSvc::create_config_builder();
    // build_loader() validates the builder is fully seeded (name + version).
    let _loader = builder.build_loader();
}

/// @covers: swe-edge-configbuilder
/// Verifies the builder carries a non-empty package name.
#[test]
fn tls_struct_svc_create_config_builder_has_non_empty_name_int_test() {
    let builder: ConfigBuilderImpl = HttpTlsSvc::create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "ConfigBuilderImpl must carry a non-empty package name"
    );
}
