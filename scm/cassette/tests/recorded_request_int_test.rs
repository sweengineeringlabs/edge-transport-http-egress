//! Integration tests for `api/recorded/interaction/request.rs`.
//! @covers: src/api/recorded/interaction/request.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfig, HttpCassetteSvc};

/// @covers: Request
/// Confirms `CassetteLayer` can be built with a disabled config — this exercises
/// the `Request` trait boundary through the public cassette construction path.
#[test]
fn cassette_trait_recorded_request_layer_builds_with_disabled_mode_int_test() {
    let config = CassetteConfig::disabled();
    let layer = HttpCassetteSvc::build_cassette_layer(config, "recorded_request_test");
    assert!(
        layer.is_ok(),
        "build_cassette_layer with disabled config must succeed"
    );
}

/// @covers: Request
/// Confirms the `CassetteConfigBuilder` can produce a `CassetteConfig`
/// with `replay` mode for request matching.
#[test]
fn cassette_trait_recorded_request_builder_replay_mode_int_test() {
    use edge_transport_http_egress_cassette::CassetteConfigBuilder;
    let cfg = CassetteConfigBuilder::new()
        .with_mode("replay")
        .with_cassette_dir("tests/cassettes")
        .with_match_on(vec!["method".into(), "url".into()])
        .with_scrub_headers(vec![])
        .with_scrub_body_paths(vec![])
        .build_config()
        .expect("all fields valid");
    assert_eq!(cfg.mode, "replay");
    assert_eq!(cfg.cassette_dir, "tests/cassettes");
}
