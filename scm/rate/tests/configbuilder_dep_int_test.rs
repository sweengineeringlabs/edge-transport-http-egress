//! Dependency coverage test for `swe-edge-configbuilder`.
//! @covers: swe-edge-configbuilder
//!
//! Rule 95: `swe-edge-configbuilder` is used in `src/` and must have
//! integration coverage with an explicit `use swe_edge_configbuilder::...` import.

use edge_transport_http_egress_rate::HttpRateSvcProcessor;
use swe_edge_configbuilder::ConfigBuilderImpl;

/// @covers: swe-edge-configbuilder
/// Confirms `HttpRateSvcProcessor::create_config_builder` returns a `ConfigBuilderImpl`
/// seeded with the crate name and version.
#[test]
fn rate_struct_svc_create_config_builder_returns_builder_int_test() {
    let builder: ConfigBuilderImpl = HttpRateSvcProcessor::create_config_builder();
    // The builder must be seeded with this crate's name; build_loader() then
    // confirms it is fully seeded (name + version) without panicking.
    assert_eq!(
        builder.name(),
        "edge-transport-http-egress-rate",
        "create_config_builder must seed the crate name"
    );
    let _loader = builder.build_loader();
}

/// @covers: swe-edge-configbuilder
/// Verifies the builder carries a non-empty package name.
#[test]
fn rate_struct_svc_create_config_builder_has_non_empty_name_int_test() {
    let builder: ConfigBuilderImpl = HttpRateSvcProcessor::create_config_builder();
    let name = builder.name();
    assert!(
        !name.is_empty(),
        "ConfigBuilderImpl must carry a non-empty package name"
    );
}
