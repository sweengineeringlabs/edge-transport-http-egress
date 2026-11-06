//! Integration tests for `StdCassetteFactory::build_cassette_layer`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette_api::{CassetteConfig, CassetteError, CassetteFactory};
use edge_transport_http_egress_cassette_std::StdCassetteFactory;

/// @covers: build_cassette_layer
/// A fixture file that is not valid YAML must be refused at build time, not at the first request.
#[test]
fn test_build_cassette_layer_with_corrupt_fixture_file_returns_parse_error_error() {
    let tmpdir = tempfile::tempdir().expect("tempdir must succeed");
    std::fs::write(
        tmpdir.path().join("corrupt.yaml"),
        "interactions: [unclosed",
    )
    .expect("write");
    let cfg = CassetteConfig {
        cassette_dir: tmpdir.path().to_string_lossy().replace('\\', "/"),
        ..CassetteConfig::default()
    };
    let result = StdCassetteFactory.build_cassette_layer(cfg, "corrupt");
    assert!(
        matches!(result, Err(CassetteError::ParseFailed(_))),
        "a corrupt fixture file must yield ParseFailed; got: {result:?}"
    );
}

/// @covers: build_cassette_layer
/// The factory with a cassette name and a temp dir config must succeed.
#[test]
fn test_build_cassette_layer_with_name_and_dir_succeeds_happy() {
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
    let result = StdCassetteFactory.build_cassette_layer(cfg, "layer_builder_test");
    assert!(
        result.is_ok(),
        "a name and a config must build a layer; got: {result:?}"
    );
}

/// @covers: build_cassette_layer
/// A config's `mode` must be reflected in the built layer's Debug output —
/// proving the factory genuinely applies the supplied config rather than
/// falling back to the default regardless of input.
#[test]
fn test_build_cassette_layer_reflects_configured_mode_happy() {
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
    let layer = StdCassetteFactory
        .build_cassette_layer(cfg, "with_config_test")
        .expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("record"),
        "the configured mode must reach the built layer; got: {dbg}"
    );
}

/// @covers: build_cassette_layer
/// The built `CassetteLayer` must produce non-empty Debug output.
#[test]
fn test_build_cassette_layer_debug_names_the_layer_edge() {
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
    let layer = StdCassetteFactory
        .build_cassette_layer(cfg, "debug_test")
        .expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        !dbg.is_empty(),
        "built CassetteLayer Debug must produce non-empty output"
    );
}
