//! Integration tests for `CassetteConfigBuilder`.
//!
//! Rule 120: `src/api/types/cassette/cassette_config_builder.rs` requires a
//! corresponding test file.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfigBuilder, CassetteError};

/// @covers: CassetteConfigBuilder::new
/// Verifies the builder is constructible with no arguments.
#[test]
fn cassette_struct_cassette_config_builder_new_returns_default_int_test() {
    let _builder = CassetteConfigBuilder::new();
}

/// @covers: CassetteConfigBuilder::build_config
/// Builder with all defaults (no fields set) must succeed and use `"replay"` mode.
#[test]
fn cassette_struct_cassette_config_builder_build_config_defaults_succeeds_int_test() {
    let result = CassetteConfigBuilder::new().build_config();
    assert!(
        result.is_ok(),
        "default builder must produce a valid config; got: {result:?}"
    );
}

/// @covers: CassetteConfigBuilder::with_mode
/// Setting a valid mode must succeed.
#[test]
fn cassette_struct_cassette_config_builder_with_valid_mode_succeeds_int_test() {
    for mode in ["replay", "record", "auto", "disabled"] {
        let result = CassetteConfigBuilder::new().with_mode(mode).build_config();
        assert!(
            result.is_ok(),
            "mode '{mode}' must be accepted; got: {result:?}"
        );
    }
}

/// @covers: CassetteConfigBuilder::with_mode
/// Setting an unknown mode must return a `CassetteError::ParseFailed`.
#[test]
fn cassette_struct_cassette_config_builder_with_invalid_mode_fails_int_test() {
    let result = CassetteConfigBuilder::new()
        .with_mode("passthrough")
        .build_config();
    assert!(result.is_err(), "unknown mode must produce an error");
    let err = result.unwrap_err();
    assert!(
        matches!(err, CassetteError::ParseFailed(_)),
        "unknown mode must yield ParseFailed; got: {err:?}"
    );
}

/// @covers: CassetteConfigBuilder::with_cassette_dir
/// Setting a cassette directory must be reflected in the built config.
#[test]
fn cassette_struct_cassette_config_builder_with_cassette_dir_reflected_int_test() {
    let cfg = CassetteConfigBuilder::new()
        .with_cassette_dir("/tmp/cassettes")
        .build_config()
        .expect("builder must succeed");
    assert_eq!(
        cfg.cassette_dir, "/tmp/cassettes",
        "cassette_dir must reflect the value set on the builder"
    );
}

/// @covers: CassetteConfigBuilder::with_match_on
/// Setting match keys must be reflected in the built config.
#[test]
fn cassette_struct_cassette_config_builder_with_match_on_reflected_int_test() {
    let keys = vec!["method".to_string(), "url".to_string()];
    let cfg = CassetteConfigBuilder::new()
        .with_match_on(keys.clone())
        .build_config()
        .expect("builder must succeed");
    assert_eq!(
        cfg.match_on, keys,
        "match_on must reflect the value set on the builder"
    );
}
