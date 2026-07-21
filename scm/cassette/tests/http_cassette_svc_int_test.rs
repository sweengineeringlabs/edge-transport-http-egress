//! Integration tests for `api/types/http_cassette_svc.rs` — the
//! `HttpCassetteSvc` facade type declaration.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfig, HttpCassetteSvc};

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_seeds_crate_name() {
    let builder = HttpCassetteSvc::create_config_builder();
    assert_eq!(builder.name(), "edge-transport-http-egress-cassette");
}

/// @covers: build_cassette_layer
#[test]
fn test_build_cassette_layer_produces_a_usable_layer() {
    let dir = tempfile::tempdir().expect("tempdir");
    let cfg = CassetteConfig {
        cassette_dir: dir.path().to_string_lossy().to_string(),
        ..CassetteConfig::swe_default().expect("baseline parses")
    };
    let layer = HttpCassetteSvc::build_cassette_layer(cfg, "http_cassette_svc_test")
        .expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("CassetteLayer"),
        "must produce a real CassetteLayer; got: {dbg}"
    );
}
