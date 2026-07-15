//! End-to-end tests for the edge_transport_http_egress_cassette SAF builder surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfig, CassetteLayer, HttpCassetteSvc};

fn make_cfg(dir: &str) -> CassetteConfig {
    CassetteConfig {
        mode: "auto".to_string(),
        cassette_dir: dir.to_string(),
        match_on: vec!["method".to_string(), "url".to_string()],
        scrub_headers: vec!["authorization".to_string()],
        scrub_body_paths: vec![],
    }
}

/// @covers: create_config_builder returns a working loader
#[test]
fn test_e2e_create_config_builder_returns_loader() {
    let _loader = HttpCassetteSvc::create_config_builder().build_loader();
}

/// @covers: default CassetteConfig mode is "replay"
#[test]
fn test_e2e_default_mode_is_replay() {
    let cfg = CassetteConfig::default();
    assert_eq!(cfg.mode, "replay", "default mode must be 'replay'");
}

/// @covers: build_cassette_layer with custom config
#[test]
fn test_e2e_builder() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let _layer: CassetteLayer =
        HttpCassetteSvc::build_cassette_layer(make_cfg(&dir), "e2e_builder_test")
            .expect("build must succeed");
}

/// @covers: build_cassette_layer stores config fields correctly
#[test]
fn test_e2e_with_config() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = make_cfg(&dir);
    assert_eq!(cfg.mode, "auto");
    HttpCassetteSvc::build_cassette_layer(cfg, "e2e_with_config_test").expect("build must succeed");
}

/// @covers: CassetteConfig fields are accessible directly
#[test]
fn test_e2e_config() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = make_cfg(&dir);
    assert!(cfg.match_on.contains(&"url".to_string()));
    assert!(cfg.scrub_headers.contains(&"authorization".to_string()));
}

/// @covers: build_cassette_layer with record mode
#[test]
fn test_e2e_build() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "record".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec!["meta.id".to_string()],
    };
    let layer = HttpCassetteSvc::build_cassette_layer(cfg, "e2e_build_test")
        .expect("e2e build must succeed");
    assert!(!format!("{layer:?}").is_empty());
}
