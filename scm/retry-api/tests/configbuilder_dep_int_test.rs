//! `ApplicationConfigBuilder` wraps a configbuilder `ConfigBuilderImpl` seeded with this
//! middleware's application name and version.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::{ConfigBuilder as _, ConfigBuilderImpl, Loader as _};
use edge_transport_http_egress_retry_api::{ApplicationConfigBuilder, RetryConfig};

/// The name configbuilder joins onto its config search directories; it must not change.
const APPLICATION_NAME: &str = "edge-transport-http-egress-retry-api";

fn seeded_directly() -> ConfigBuilderImpl {
    configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_name(APPLICATION_NAME)
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// @covers: new
#[test]
fn api_struct_application_config_builder_matches_a_directly_seeded_builder_int_test() {
    let direct = seeded_directly();
    let wrapped = ApplicationConfigBuilder::new();
    assert_eq!(wrapped.name(), direct.name());
    assert_eq!(wrapped.version(), direct.version());
    assert_eq!(wrapped.name(), APPLICATION_NAME);
}

/// @covers: new
#[test]
fn api_struct_application_config_builder_default_equals_new_int_test() {
    let default = ApplicationConfigBuilder::default();
    let new = ApplicationConfigBuilder::new();
    assert_eq!(default.name(), new.name());
    assert_eq!(default.version(), new.version());
}

/// @covers: with_config_dir
#[test]
fn api_struct_application_config_builder_loads_the_shipped_retry_section_int_test() {
    let loader = ApplicationConfigBuilder::new()
        .with_config_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/config"))
        .build_loader()
        .expect("the shipped config directory must build a loader");
    let cfg: RetryConfig = loader
        .load_section("retry")
        .expect("the shipped [retry] section must load");
    assert_eq!(
        cfg.max_retries, 3,
        "the shipped config seeds max_retries = 3"
    );
}

/// @covers: with_config_dir
#[test]
fn api_struct_application_config_builder_pointed_at_a_dir_without_config_fails_int_test() {
    let loader = ApplicationConfigBuilder::new()
        .with_config_dir(env!("CARGO_MANIFEST_DIR"))
        .build_loader()
        .expect("building the loader itself must still succeed");
    let cfg: Result<RetryConfig, _> = loader.load_section("retry");
    assert!(
        cfg.is_err(),
        "a directory with no application.toml must not yield a [retry] section"
    );
}
