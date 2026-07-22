//! Integration tests for `create_config_builder` in `edge_transport_http_egress_cache`.

use edge_transport_http_egress_cache::HttpCacheSvcProcessor;

/// @covers: HttpCacheSvcProcessor::create_config_builder — dep coverage for swe-edge-configbuilder
#[test]
fn cache_struct_create_config_builder_returns_seeded_builder_int_test() {
    let builder = HttpCacheSvcProcessor::create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must be seeded with crate name"
    );
    assert!(
        !builder.version().is_empty(),
        "builder must be seeded with crate version"
    );
}
