//! Integration tests for `CassetteLayerBuilder`.
//!
//! Rule 120: `src/api/types/cassette/cassette_layer_builder.rs` requires a
//! corresponding test file.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfig, CassetteError, CassetteLayerBuilder};

/// @covers: new
/// A freshly-constructed builder must reject `build_layer()` until a cassette
/// name is supplied — proving `new()` starts in an unconfigured state rather
/// than silently succeeding.
#[test]
fn cassette_struct_cassette_layer_builder_new_returns_default_int_test() {
    let result = CassetteLayerBuilder::new().build_layer();
    assert!(
        matches!(result, Err(CassetteError::ParseFailed(_))),
        "a builder from new() must require a cassette name; got: {result:?}"
    );
}

/// @covers: build_layer
/// Building without a cassette name must fail with `CassetteError::ParseFailed`.
#[test]
fn cassette_struct_cassette_layer_builder_build_layer_missing_name_fails_int_test() {
    let result = CassetteLayerBuilder::new().build_layer();
    assert!(
        result.is_err(),
        "missing cassette_name must produce an error"
    );
    let err = result.unwrap_err();
    assert!(
        matches!(err, CassetteError::ParseFailed(_)),
        "missing cassette_name must yield ParseFailed; got: {err:?}"
    );
}

/// @covers: with_cassette_name
/// Builder with a cassette name and a temp dir config must succeed.
#[test]
fn cassette_struct_cassette_layer_builder_with_name_and_dir_succeeds_int_test() {
    let tmpdir = tempfile::tempdir().expect("tempdir must succeed");
    let dir = tmpdir
        .path()
        .to_str()
        .expect("path must be utf-8")
        .replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "replay".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string(), "url".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    let result = CassetteLayerBuilder::new()
        .with_config(cfg)
        .with_cassette_name("layer_builder_test")
        .build_layer();
    assert!(
        result.is_ok(),
        "builder with name + config must succeed; got: {result:?}"
    );
}

/// @covers: with_config
/// A config's `mode` must be reflected in the built layer's Debug output —
/// proving `with_config` genuinely applies the supplied config rather than
/// falling back to the default regardless of input.
#[test]
fn cassette_struct_cassette_layer_builder_with_config_reflects_mode_int_test() {
    let tmpdir = tempfile::tempdir().expect("tempdir must succeed");
    let dir = tmpdir
        .path()
        .to_str()
        .expect("path must be utf-8")
        .replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "record".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    let layer = CassetteLayerBuilder::new()
        .with_config(cfg)
        .with_cassette_name("with_config_test")
        .build_layer()
        .expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("record"),
        "with_config's mode must reach the built layer; got: {dbg}"
    );
}

/// @covers: build_layer
/// The built `CassetteLayer` must produce non-empty Debug output.
#[test]
fn cassette_struct_cassette_layer_builder_built_layer_debug_non_empty_int_test() {
    let tmpdir = tempfile::tempdir().expect("tempdir must succeed");
    let dir = tmpdir
        .path()
        .to_str()
        .expect("path must be utf-8")
        .replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "auto".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    let layer = CassetteLayerBuilder::new()
        .with_config(cfg)
        .with_cassette_name("debug_test")
        .build_layer()
        .expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        !dbg.is_empty(),
        "built CassetteLayer Debug must produce non-empty output"
    );
}
