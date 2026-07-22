//! Integration tests for `CassetteModeRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{
    CassetteConfig, CassetteModeRequest, HttpCassette, HttpCassetteSvc,
};

/// @covers: mode
#[test]
fn test_mode_reflects_configured_replay_mode_happy() {
    let dir = tempfile::tempdir().expect("tempdir");
    let cfg = CassetteConfig {
        mode: "replay".to_string(),
        cassette_dir: dir.path().to_string_lossy().to_string(),
        ..CassetteConfig::swe_default().expect("baseline parses")
    };
    let layer = HttpCassetteSvc::build_cassette_layer(cfg, "mode_request_happy")
        .expect("build must succeed");
    let resp = layer.mode(CassetteModeRequest).expect("infallible");
    assert_eq!(resp.value, "replay");
}

/// @covers: mode
#[test]
fn test_mode_reflects_configured_disabled_mode_edge() {
    let dir = tempfile::tempdir().expect("tempdir");
    let cfg = CassetteConfig {
        mode: "disabled".to_string(),
        cassette_dir: dir.path().to_string_lossy().to_string(),
        ..CassetteConfig::swe_default().expect("baseline parses")
    };
    let layer = HttpCassetteSvc::build_cassette_layer(cfg, "mode_request_edge")
        .expect("build must succeed");
    let resp = layer.mode(CassetteModeRequest).expect("infallible");
    assert_eq!(
        resp.value, "disabled",
        "the boundary 'disabled' mode must round-trip unchanged"
    );
}
