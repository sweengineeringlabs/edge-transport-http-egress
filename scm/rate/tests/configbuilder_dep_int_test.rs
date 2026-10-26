//! Dependency coverage test for `configbuilder`.
//! @covers: configbuilder
//!
//! Rule 95: `configbuilder` is used in `src/` and must have
//! integration coverage with an explicit `use configbuilder::...` import.

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigBuilderImpl};
use edge_transport_http_egress_rate::HttpRateSvcProcessor;

/// @covers: configbuilder
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

/// @covers: configbuilder
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
