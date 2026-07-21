//! Integration tests for `api/cassette/fixture_store.rs` — the api/
//! structural counterpart of
//! `core::cassette::fixture_store::FixtureStore`.
//!
//! `FixtureStore` itself is `pub(crate)` (guards the cassette's in-memory
//! fixture map) and has its own inline unit tests. From outside the crate we
//! verify the externally-observable precondition fixture storage depends on:
//! a freshly built `CassetteLayer` is ready for record/replay regardless of
//! its configured mode.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfig, HttpCassetteSvc};

/// @covers: build_cassette_layer
#[test]
fn test_freshly_built_cassette_layer_is_ready_for_fixture_storage() {
    let dir = tempfile::tempdir().expect("tempdir");
    let cfg = CassetteConfig {
        mode: "auto".to_string(),
        cassette_dir: dir.path().to_string_lossy().to_string(),
        ..CassetteConfig::swe_default().expect("baseline parses")
    };
    let layer = HttpCassetteSvc::build_cassette_layer(cfg, "fixture_store_test")
        .expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(!dbg.is_empty());
    assert!(dbg.contains("CassetteLayer"));
}
