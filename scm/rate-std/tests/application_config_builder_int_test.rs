//! Integration tests for `create_config_builder` in `edge_transport_http_egress_rate`.

use configbuilder::ConfigBuilder as _;
use edge_transport_http_egress_rate_api::RateFactory;
use edge_transport_http_egress_rate_std::StdRateFactory;

/// @covers: HttpRateSvcProcessor::create_config_builder — dep coverage for configbuilder
#[test]
fn std_struct_create_config_builder_returns_seeded_builder_int_test() {
    let builder = StdRateFactory.create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must be seeded with crate name"
    );
    assert!(
        !builder.version().is_empty(),
        "builder must be seeded with crate version"
    );
}
