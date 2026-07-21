//! Dependency coverage test for `swe-edge-configbuilder`.
//! Verifies that the configbuilder integration works through the
//! cache public API.

use edge_transport_http_egress_cache::HttpCacheSvcProcessor;
use swe_edge_configbuilder::ConfigBuilderImpl;

/// @covers: swe-edge-configbuilder
#[test]
fn cache_struct_dep_configbuilder_create_config_builder_returns_builder_int_test() {
    let builder: ConfigBuilderImpl = HttpCacheSvcProcessor::create_config_builder();
    // The builder must be seeded with this crate's own name and version via
    // `env!` — proving `create_config_builder` actually wires them in, not
    // just that a default builder was returned.
    assert_eq!(
        builder.name(),
        "edge-transport-http-egress-cache",
        "builder must carry this crate's package name"
    );
    assert!(
        !builder.version().is_empty(),
        "builder must carry this crate's package version"
    );
}
