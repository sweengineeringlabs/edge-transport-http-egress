//! Integration tests for `HttpTransportSvc::create_config_builder`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpTransportSvc;

/// @covers: create_config_builder
/// `create_config_builder` returns a `ConfigBuilderImpl` pre-seeded with this
/// crate's package name and version. Assert the builder carries exactly those
/// seeds so a stub factory returning an unseeded builder would be caught.
#[test]
fn test_create_config_builder_constructs_application_config_builder() {
    let builder = HttpTransportSvc::create_config_builder();
    assert_eq!(
        builder.name(),
        env!("CARGO_PKG_NAME"),
        "builder must be seeded with this crate's package name"
    );
    assert_eq!(
        builder.version(),
        env!("CARGO_PKG_VERSION"),
        "builder must be seeded with this crate's package version"
    );
}
