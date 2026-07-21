//! Integration tests for `HttpTransportSvc`.
//!
//! Rule 120: `src/api/types/http_transport_svc.rs` requires a corresponding
//! test file.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpTransportSvc;

/// @covers: HttpTransportSvc::create_config_builder
/// The factory type must expose a `create_config_builder` method that returns
/// a builder seeded with the package name.
#[test]
fn transport_struct_http_transport_svc_create_config_builder_has_name_int_test() {
    let builder = HttpTransportSvc::create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "HttpTransportSvc config builder must carry a non-empty package name"
    );
}

/// @covers: HttpTransportSvc::create_config_builder
/// The builder must be seeded with this crate's version and then produce a
/// loader without panicking.
#[test]
fn transport_struct_http_transport_svc_create_config_builder_builds_loader_int_test() {
    let builder = HttpTransportSvc::create_config_builder();
    assert_eq!(
        builder.version(),
        env!("CARGO_PKG_VERSION"),
        "config builder must be seeded with this crate's package version"
    );
    let _loader = builder.build_loader();
}
