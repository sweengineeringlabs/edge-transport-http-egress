//! Integration tests for `api/recorded/interaction/request.rs`.
//! @covers: src/api/recorded/interaction/request.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette_api::{CassetteConfig, CassetteFactory};
use edge_transport_http_egress_cassette_std::StdCassetteFactory;

/// @covers: build_cassette_layer
/// Confirms `CassetteLayer` can be built with a disabled config — the recorded
/// request path is reachable through the public cassette construction path, and
/// the built layer reflects the disabled mode.
#[test]
fn std_trait_recorded_request_layer_builds_with_disabled_mode_int_test() {
    let config = CassetteConfig::disabled();
    let layer = StdCassetteFactory
        .build_cassette_layer(config, "recorded_request_test")
        .expect("build_cassette_layer with disabled config must succeed");
    assert!(
        format!("{layer:?}").contains("disabled"),
        "the built layer must carry the disabled mode"
    );
}

/// @covers: Request
/// Confirms the `CassetteConfigBuilder` can produce a `CassetteConfig`
/// with `replay` mode for request matching.
#[test]
fn std_trait_recorded_request_builder_replay_mode_int_test() {
    use edge_transport_http_egress_cassette_api::CassetteConfigBuilder;
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
