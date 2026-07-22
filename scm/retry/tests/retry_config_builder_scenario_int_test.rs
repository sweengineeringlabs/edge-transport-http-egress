//! Scenario coverage for `Processor::new_config_builder` and
//! `Processor::new_app_config_builder`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::{HttpRetrySvc, Processor};

/// @covers: new_config_builder
#[test]
fn test_new_config_builder_seeds_swe_default_max_retries_happy() {
    let cfg = HttpRetrySvc::new_config_builder()
        .build()
        .expect("default builder config must pass validation");
    assert_eq!(cfg.max_retries, 3);
}

/// @covers: new_config_builder
#[test]
fn test_new_config_builder_produces_independent_instances_edge() {
    let a = HttpRetrySvc::new_config_builder()
        .build()
        .expect("must build");
    let b = HttpRetrySvc::new_config_builder()
        .build()
        .expect("must build");
    assert_eq!(a.max_retries, b.max_retries);
}

/// @covers: new_config_builder
#[test]
fn test_new_config_builder_is_mutable_before_build_edge() {
    let cfg = HttpRetrySvc::new_config_builder()
        .max_retries(7)
        .build()
        .expect("must build");
    assert_eq!(
        cfg.max_retries, 7,
        "the builder returned by new_config_builder must accept further mutation"
    );
}

/// @covers: new_app_config_builder
#[test]
fn test_new_app_config_builder_seeds_crate_name_happy() {
    let builder = HttpRetrySvc::new_app_config_builder();
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
}

/// @covers: new_app_config_builder
#[test]
fn test_new_app_config_builder_seeds_crate_version_edge() {
    let builder = HttpRetrySvc::new_app_config_builder();
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
}

/// @covers: new_app_config_builder
#[test]
fn test_new_app_config_builder_produces_independent_instances_edge() {
    let a = HttpRetrySvc::new_app_config_builder();
    let b = HttpRetrySvc::new_app_config_builder();
    assert_eq!(a.name(), b.name());
    assert_eq!(a.version(), b.version());
}

/// @covers: new_config_builder
#[test]
fn test_new_config_builder_configured_to_an_invalid_state_fails_to_build_error() {
    let result = HttpRetrySvc::new_config_builder().multiplier(0.0).build();
    assert!(
        result.is_err(),
        "the builder returned by new_config_builder must still enforce validation on build()"
    );
}

/// @covers: new_app_config_builder
#[test]
fn test_new_app_config_builder_pointed_at_a_missing_config_dir_fails_to_load_error() {
    let result = HttpRetrySvc::new_app_config_builder()
        .with_config_dir(env!("CARGO_MANIFEST_DIR"))
        .build_loader()
        .expect("build_loader itself must still succeed")
        .load_section::<edge_transport_http_egress_retry::RetryConfig>("retry");
    assert!(
        result.is_err(),
        "a config dir with no application.toml must fail to load [retry]"
    );
}
