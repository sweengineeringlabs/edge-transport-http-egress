//! Integration tests verifying `swe-edge-configbuilder` coverage through the
//! `HttpBreakerSvcProcessor::create_config_builder()` SAF entry point.
//!
//! Rule 95: `swe-edge-configbuilder` is used in `src/` and must have
//! integration/e2e coverage.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{BreakerConfig, HttpBreakerSvcProcessor};
use swe_edge_configbuilder::ConfigBuilderImpl;

/// @covers: create_config_builder
/// Verifies the returned loader is genuinely functional — not just that
/// `build_loader()` itself returns `Ok` — by actually attempting a section
/// load through it and checking the outcome is a real, well-formed
/// `ConfigError` (no config file exists in the test environment), not a
/// panic or a silently-succeeding stub.
#[test]
fn breaker_struct_svc_create_config_builder_returns_loader_int_test() {
    let loader = HttpBreakerSvcProcessor::create_config_builder()
        .build_loader()
        .expect("build_loader must succeed with no config dir configured");
    let result: Result<BreakerConfig, _> = loader.load_section("breaker");
    assert!(
        result.is_err(),
        "loading a section with no config file present must be a real error, not a silent default"
    );
}

/// @covers: create_config_builder
/// Verifies the crate's own name and version (via `env!`) are actually
/// injected into the builder — not just that the builder is constructable.
#[test]
fn breaker_struct_svc_create_config_builder_has_crate_name_int_test() {
    let builder: ConfigBuilderImpl = HttpBreakerSvcProcessor::create_config_builder();
    assert_eq!(
        builder.name(),
        "edge-transport-http-egress-breaker",
        "builder must be seeded with this crate's own package name"
    );
    assert!(
        !builder.version().is_empty(),
        "builder must be seeded with this crate's own package version"
    );
}
