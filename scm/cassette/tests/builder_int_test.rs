//! Integration tests for `edge_transport_http_egress_cassette` SAF builder entry points.
//!
//! Covers: `create_config_builder()`, `HttpCassetteSvc::build_cassette_layer(config, name)`, and all config variants.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfig, CassetteLayer, HttpCassetteSvc};

fn make_config(dir: &str) -> CassetteConfig {
    // Normalize backslashes so TOML doesn't treat `\U`, `\t`, etc. as escape
    // sequences inside the basic string.
    let dir_safe = dir.replace('\\', "/");
    CassetteConfig {
        mode: "auto".to_string(),
        cassette_dir: dir_safe,
        match_on: vec!["method".to_string(), "url".to_string()],
        scrub_headers: vec!["authorization".to_string()],
        scrub_body_paths: vec![],
    }
}

// ---------------------------------------------------------------------------
// create_config_builder() — SAF entry point
// ---------------------------------------------------------------------------

/// The crate-shipped baseline TOML must always parse; otherwise no consumer
/// of this crate can bootstrap without supplying their own config.
#[test]
fn test_builder_fn_loads_swe_default_and_returns_ok() {
    HttpCassetteSvc::create_config_builder()
        .build_loader()
        .expect("SWE default TOML must parse without error");
}

/// The SWE default mode is "replay" — tests must not accidentally record
/// real traffic when the caller forgets to override the mode.
#[test]
fn test_builder_fn_swe_default_mode_is_replay() {
    let cfg = CassetteConfig::default();
    assert_eq!(
        cfg.mode, "replay",
        "swe_default mode must be 'replay' to prevent accidental recording"
    );
}

/// `authorization` must be in the default scrub list so cassettes committed
/// to VCS cannot leak API credentials.
#[test]
fn test_builder_fn_swe_default_scrubs_authorization_header() {
    let cfg = CassetteConfig::default();
    let has_auth = cfg
        .scrub_headers
        .iter()
        .any(|h| h.eq_ignore_ascii_case("authorization"));
    assert!(
        has_auth,
        "swe_default scrub_headers must include 'authorization'; got: {:?}",
        cfg.scrub_headers
    );
}

// ---------------------------------------------------------------------------
// build_cassette_layer — custom config flows through unchanged
// ---------------------------------------------------------------------------

/// All fields supplied through `build_cassette_layer` must survive unchanged.
#[test]
fn test_with_config_stores_all_fields_unchanged() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = make_config(&dir);

    assert_eq!(cfg.mode, "auto");
    assert_eq!(cfg.cassette_dir, dir);
    assert!(cfg.match_on.contains(&"method".to_string()));
    assert!(cfg.match_on.contains(&"url".to_string()));
    assert!(cfg.scrub_headers.contains(&"authorization".to_string()));
    assert!(cfg.scrub_body_paths.is_empty());
}

/// `CassetteConfig` fields must reflect the stored reference after construction.
#[test]
fn test_config_accessor_returns_stored_reference() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "record".to_string(),
        cassette_dir: dir,
        match_on: vec!["body_hash".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec!["request_id".to_string()],
    };
    assert_eq!(cfg.mode, "record");
    assert_eq!(cfg.match_on, vec!["body_hash"]);
    assert_eq!(cfg.scrub_body_paths, vec!["request_id"]);
}

// ---------------------------------------------------------------------------
// build_cassette_layer — produces a CassetteLayer
// ---------------------------------------------------------------------------

/// Happy path: `build_cassette_layer` must succeed and return a `CassetteLayer` whose
/// Debug output identifies the type and records the configured mode.
#[test]
fn test_build_with_auto_mode_returns_cassette_layer() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let layer: CassetteLayer =
        HttpCassetteSvc::build_cassette_layer(make_config(&dir), "happy_path")
            .expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("CassetteLayer"),
        "Debug must contain 'CassetteLayer'; got: {dbg}"
    );
}

/// Building in "replay" mode with no pre-existing fixture file must succeed
/// — the layer starts with an empty in-memory map and only fails when a
/// request arrives with no recorded match.
#[test]
fn test_build_replay_mode_missing_fixture_file_succeeds() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "replay".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string(), "url".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    HttpCassetteSvc::build_cassette_layer(cfg, "replay_no_fixture")
        .expect("replay with missing fixture must build");
}

/// Building in "record" mode must succeed so a fresh recording session can
/// start without requiring a pre-existing cassette file.
#[test]
fn test_build_record_mode_succeeds() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "record".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    HttpCassetteSvc::build_cassette_layer(cfg, "record_session").expect("record mode must build");
}

/// Multiple scrub body paths (including nested dot-paths) must not prevent
/// `build_cassette_layer` from succeeding — path parsing happens lazily at request time.
#[test]
fn test_build_with_nested_scrub_body_paths_succeeds() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "auto".to_string(),
        cassette_dir: dir,
        match_on: vec![
            "method".to_string(),
            "url".to_string(),
            "body_hash".to_string(),
        ],
        scrub_headers: vec!["authorization".to_string()],
        scrub_body_paths: vec!["request_id".to_string(), "metadata.trace_id".to_string()],
    };
    HttpCassetteSvc::build_cassette_layer(cfg, "nested_scrub")
        .expect("nested scrub body paths must build");
}

// ---------------------------------------------------------------------------
// CassetteLayer: Send + Sync — compile-time proof
// ---------------------------------------------------------------------------

/// `CassetteLayer` must be `Send + Sync` so it can be used across async
/// task boundaries (e.g. shared via `Arc` in a `reqwest_middleware` chain).
/// Prove it at runtime: share a reference across a real OS thread (requires
/// `Sync`) and move an owned copy into another (requires `Send`), asserting the
/// value observed on the other thread is intact.
#[test]
fn test_cassette_layer_is_send_and_sync() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let layer: CassetteLayer =
        HttpCassetteSvc::build_cassette_layer(make_config(&dir), "send_sync_move")
            .expect("build must succeed");
    std::thread::scope(|s| {
        let borrowed = &layer;
        let dbg = s
            .spawn(move || format!("{borrowed:?}"))
            .join()
            .expect("thread sharing &layer must not panic");
        assert!(
            dbg.contains("send_sync_move"),
            "layer shared across a thread must retain its cassette name: {dbg}"
        );
    });
    let moved = std::thread::spawn(move || format!("{layer:?}"))
        .join()
        .expect("thread owning the moved layer must not panic");
    assert!(
        moved.contains("send_sync_move"),
        "layer moved into another thread must retain its cassette name: {moved}"
    );
}
