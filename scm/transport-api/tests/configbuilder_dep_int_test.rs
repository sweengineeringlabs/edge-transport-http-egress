//! Direct integration coverage for the contract's configbuilder dependency.

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigBuilderImpl};
use edge_transport_http_egress_transport_api::ApplicationConfigBuilder;

/// @covers: configbuilder
#[test]
fn test_configbuilder_dependency_creates_seeded_builder_happy() {
    let builder: ConfigBuilderImpl = configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_name("edge-transport-http-egress-transport-api")
        .with_version("1.2.3");
    assert_eq!(builder.name(), "edge-transport-http-egress-transport-api");
    assert_eq!(builder.version(), "1.2.3");
    let _loader = builder.build_loader();
}

/// @covers: with_name
#[test]
fn test_application_config_builder_keeps_the_seeded_name_and_version_edge() {
    let builder = ApplicationConfigBuilder::new()
        .with_name("edge-transport-http-egress-transport-api")
        .with_version("0.0.1");
    assert_eq!(builder.name(), "edge-transport-http-egress-transport-api");
    assert_eq!(builder.version(), "0.0.1");
}
