//! Direct integration coverage for the transport's configbuilder dependency.

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigBuilderImpl};
use edge_transport_http_egress_transport::HttpTransportSvc;

/// @covers: configbuilder
#[test]
fn transport_type_configbuilder_dependency_creates_seeded_builder_int_test() {
    let builder: ConfigBuilderImpl = configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"));
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
    let _loader = builder.build_loader();

    let facade = HttpTransportSvc::create_config_builder();
    assert_eq!(facade.name(), env!("CARGO_PKG_NAME"));
    assert_eq!(facade.version(), env!("CARGO_PKG_VERSION"));
}
