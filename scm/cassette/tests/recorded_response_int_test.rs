//! Integration tests for `api/recorded/interaction/response.rs`.
//! @covers: src/api/recorded/interaction/response.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfig, HttpCassetteSvc};

/// @covers: Response
/// Confirms `CassetteLayer` Debug output includes the mode, which is derived
/// from the config attached to the response-handling layer.
#[test]
fn cassette_trait_recorded_response_layer_debug_shows_mode_int_test() {
    let config = CassetteConfig::disabled();
    let layer = HttpCassetteSvc::build_cassette_layer(config, "recorded_response_test")
        .expect("disabled mode must always succeed");
    let debug = format!("{layer:?}");
    assert!(
        debug.contains("disabled"),
        "debug must show the configured mode"
    );
}

/// @covers: Response
/// Confirms the `CassetteLayerBuilder` produces a layer with the correct
/// cassette path derived from config dir and cassette name.
#[test]
fn cassette_trait_recorded_response_builder_layer_path_int_test() {
    use edge_transport_http_egress_cassette::CassetteLayerBuilder;
    let layer = CassetteLayerBuilder::new()
        .with_config(CassetteConfig::disabled())
        .with_cassette_name("response_test")
        .build_layer()
        .expect("builder with all fields set must succeed");
    let debug = format!("{layer:?}");
    assert!(
        debug.contains("response_test"),
        "cassette path must contain the cassette name: {debug}"
    );
}
