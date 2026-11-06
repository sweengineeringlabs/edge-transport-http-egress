//! Integration tests for `create_config_builder` in `edge_transport_http_egress_cassette_std`.

use configbuilder::ConfigBuilder as _;
use edge_transport_http_egress_cassette_api::CassetteFactory;
use edge_transport_http_egress_cassette_std::StdCassetteFactory;

/// @covers: HttpCassetteSvc::create_config_builder — dep coverage for configbuilder
#[test]
fn std_struct_create_config_builder_returns_seeded_builder_int_test() {
    let builder = StdCassetteFactory.create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must be seeded with crate name"
    );
    assert!(
        !builder.version().is_empty(),
        "builder must be seeded with crate version"
    );
}
