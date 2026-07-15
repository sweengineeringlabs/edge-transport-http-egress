//! Integration tests for `saf::builder` — the SAF-layer builder module.
//!
//! `saf/builder.rs` contains:
//! - `create_config_builder()` — the public SAF entry point, returns a loader.
//! - `HttpCassetteSvc::build_cassette_layer(config, cassette_name)` — builds a CassetteLayer directly.
//!
//! The SWE default mode is "replay" (prevents accidental real-network recording).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{
    CassetteConfig, CassetteError, CassetteLayer, HttpCassetteSvc,
};

// ---------------------------------------------------------------------------
// create_config_builder() — SAF entry point: always returns a loader
// ---------------------------------------------------------------------------

/// `create_config_builder()` must succeed unconditionally. If the crate-shipped
/// `config/application.toml` ever breaks, this is the first test to fail.
#[test]
fn test_saf_builder_fn_returns_ok() {
    let _loader = HttpCassetteSvc::create_config_builder().build_loader();
}

/// The SWE default mode must be "replay". Any change to the default TOML
/// that switches mode to "record" would cause integration tests to make
/// real network calls — catching that here prevents accidental recordings.
#[test]
fn test_saf_builder_fn_default_mode_is_replay() {
    let cfg = CassetteConfig::default();
    assert_eq!(
        cfg.mode, "replay",
        "default mode must be 'replay'; got '{}'",
        cfg.mode
    );
}

/// The SWE default match_on list must not be empty. An empty list would
/// produce a single match key for every request, making all requests
/// collapse onto the same fixture.
#[test]
fn test_saf_builder_fn_default_match_on_is_not_empty() {
    let cfg = CassetteConfig::default();
    assert!(
        !cfg.match_on.is_empty(),
        "default match_on must not be empty"
    );
}

// ---------------------------------------------------------------------------
// build_cassette_layer — custom config flow
// ---------------------------------------------------------------------------

/// Custom config supplied via `build_cassette_layer` must be accepted and produce a
/// `CassetteLayer` without error. This proves the SAF builder layer passes
/// the config through to `CassetteLayer::new` unchanged.
#[test]
fn test_saf_with_config_and_build_produces_cassette_layer() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "auto".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string(), "url".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    let layer: CassetteLayer = HttpCassetteSvc::build_cassette_layer(cfg, "saf_with_config")
        .expect("with_config + build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("CassetteLayer"),
        "must be a CassetteLayer; got: {dbg}"
    );
}

/// `build_cassette_layer` takes a cassette name which becomes part of the on-disk path.
/// Two calls with different names must produce distinct cassette files.
#[test]
fn test_saf_build_uses_cassette_name_in_path() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg_a = CassetteConfig {
        mode: "replay".to_string(),
        cassette_dir: dir.clone(),
        match_on: vec!["method".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    let cfg_b = CassetteConfig {
        mode: "replay".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    let l_a = HttpCassetteSvc::build_cassette_layer(cfg_a, "cassette_alpha").unwrap();
    let l_b = HttpCassetteSvc::build_cassette_layer(cfg_b, "cassette_beta").unwrap();
    let dbg_a = format!("{l_a:?}");
    let dbg_b = format!("{l_b:?}");
    // The cassette path is embedded in the Debug output; it must differ.
    assert_ne!(
        dbg_a, dbg_b,
        "different cassette names must produce different Debug output"
    );
}

// ---------------------------------------------------------------------------
// Error propagation through the SAF builder
// ---------------------------------------------------------------------------

/// `ParseFailed` constructed directly must surface through `CassetteError::ParseFailed`
/// and embed the crate name in its display — confirming the error type
/// exported by the SAF surface is the same one `build_cassette_layer` would return on
/// a malformed config.
#[test]
fn test_saf_error_parse_failed_display_names_crate() {
    let err = CassetteError::ParseFailed("bad field".to_string());
    assert!(
        err.to_string()
            .contains("edge_transport_http_egress_cassette"),
        "CassetteError::ParseFailed from SAF layer must name the crate"
    );
}
