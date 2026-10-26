//! Dependency coverage test for `configbuilder`.
//! @covers: configbuilder
//!
//! Rule 95: `configbuilder` is used in `src/` and must have
//! integration coverage with an explicit `use configbuilder::...` import.

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigBuilderImpl};
use edge_transport_http_egress_retry::{HttpRetrySvc, Processor};

/// @covers: configbuilder
/// Confirms the crate's `ApplicationConfigBuilder` genuinely wraps a
/// `ConfigBuilderImpl` seeded with the crate name and version — constructed
/// directly here (bypassing the crate's own facade) to exercise the external
/// dependency's real API surface.
#[test]
fn retry_struct_svc_create_config_builder_returns_builder_int_test() {
    let builder: ConfigBuilderImpl = configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"));
    assert!(
        !builder.name().is_empty() && !builder.version().is_empty(),
        "builder must be seeded with name and version"
    );
    let name = builder.name().to_string();
    let version = builder.version().to_string();
    let _loader = builder.build_loader();

    // Cross-check: the crate's own facade must carry the same identity.
    let facade = HttpRetrySvc::new_app_config_builder();
    assert_eq!(facade.name(), name);
    assert_eq!(facade.version(), version);
}

/// @covers: configbuilder
/// Verifies the builder carries a non-empty package name.
#[test]
fn retry_struct_svc_create_config_builder_has_non_empty_name_int_test() {
    let builder: ConfigBuilderImpl = configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"));
    assert!(
        !builder.name().is_empty(),
        "ConfigBuilderImpl must carry a non-empty package name"
    );
}
