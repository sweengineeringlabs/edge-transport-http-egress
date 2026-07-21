//! Integration tests for [`HttpCassetteFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{
    CassetteConfig, CassetteModeRequest, HttpCassetteFactory, HttpCassetteSvc,
};

/// @covers: from_layer
#[test]
fn test_from_layer_upcasts_to_mode_inspection_happy() {
    let dir = tempfile::tempdir().expect("tempdir");
    let cfg = CassetteConfig {
        mode: "record".to_string(),
        cassette_dir: dir.path().to_string_lossy().to_string(),
        ..CassetteConfig::swe_default().expect("baseline parses")
    };
    let layer = HttpCassetteSvc::build_cassette_layer(cfg, "from_layer_happy").expect("build ok");
    let cassette = HttpCassetteFactory::from_layer(layer);
    let resp = cassette
        .mode(CassetteModeRequest)
        .expect("upcast must genuinely work");
    assert_eq!(resp.value, "record");
}

/// @covers: from_layer
#[test]
fn test_from_layer_reflects_a_non_default_mode_value_edge() {
    let dir = tempfile::tempdir().expect("tempdir");
    let cfg = CassetteConfig {
        mode: "auto".to_string(),
        cassette_dir: dir.path().to_string_lossy().to_string(),
        ..CassetteConfig::swe_default().expect("baseline parses")
    };
    let layer = HttpCassetteSvc::build_cassette_layer(cfg, "from_layer_edge").expect("build ok");
    let cassette = HttpCassetteFactory::from_layer(layer);
    let resp = cassette.mode(CassetteModeRequest).expect("must succeed");
    assert_eq!(resp.value, "auto");
}
