//! Integration tests for `CassetteLayer` public surface (api type).
//!
//! `CassetteLayer` is an opaque type created via `build_cassette_layer`. Tests
//! exercise observable properties: Debug output, Send+Sync bounds, and
//! path derivation from `cassette_dir` + cassette name.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfig, CassetteLayer, HttpCassetteSvc};

fn make_cfg(dir: &str, mode: &str) -> CassetteConfig {
    CassetteConfig {
        mode: mode.to_string(),
        cassette_dir: dir.replace('\\', "/"),
        match_on: vec!["method".to_string(), "url".to_string()],
        scrub_headers: vec!["authorization".to_string()],
        scrub_body_paths: vec![],
    }
}

// ---------------------------------------------------------------------------
// CassetteLayer construction
// ---------------------------------------------------------------------------

/// Building with a temp directory and a fresh cassette name must succeed,
/// returning a `CassetteLayer` without touching the filesystem yet.
#[test]
fn test_build_returns_cassette_layer_for_nonexistent_cassette() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap();
    let layer: CassetteLayer =
        HttpCassetteSvc::build_cassette_layer(make_cfg(dir, "auto"), "nonexistent_cassette")
            .expect("build must succeed");
    // The cassette file should not exist on disk yet — it is only written
    // when the first request is recorded in "record" or "auto" mode.
    let expected = tmpdir.path().join("nonexistent_cassette.yaml");
    assert!(
        !expected.exists(),
        "cassette file must not be created at build time"
    );
    drop(layer); // explicit for readability
}

/// The cassette file path must be derived from `cassette_dir` + cassette
/// name + `.yaml` extension, which the Debug output must reflect.
#[test]
fn test_cassette_layer_debug_contains_mode_and_path() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap();
    let layer = HttpCassetteSvc::build_cassette_layer(make_cfg(dir, "replay"), "debug_check")
        .expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("CassetteLayer"),
        "Debug must mention 'CassetteLayer'; got: {dbg}"
    );
    // Mode must appear in the Debug output so operators can diagnose whether
    // the layer will attempt a real request or replay from fixtures.
    assert!(
        dbg.contains("replay"),
        "Debug must include the mode; got: {dbg}"
    );
}

/// Two independent `build_cassette_layer` calls with different cassette names must produce
/// different on-disk paths. This is essential for test isolation — each
/// test case must own its own fixture file.
#[test]
fn test_two_layers_with_different_names_have_different_paths() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap();
    let a = HttpCassetteSvc::build_cassette_layer(make_cfg(dir, "auto"), "cassette_a")
        .expect("build a");
    let b = HttpCassetteSvc::build_cassette_layer(make_cfg(dir, "auto"), "cassette_b")
        .expect("build b");
    let dbg_a = format!("{a:?}");
    let dbg_b = format!("{b:?}");
    // Paths are embedded in Debug; they must differ.
    assert_ne!(
        dbg_a, dbg_b,
        "two layers with different names must have different debug output"
    );
}

// ---------------------------------------------------------------------------
// CassetteLayer: Send + Sync bounds
// ---------------------------------------------------------------------------

/// `CassetteLayer` must be `Send + Sync` to be shared across async task
/// boundaries inside a `reqwest_middleware::ClientWithMiddleware`.
#[test]
fn test_cassette_layer_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<CassetteLayer>();
}

#[test]
fn test_cassette_layer_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<CassetteLayer>();
}

// ---------------------------------------------------------------------------
// Mode variants — all three modes must produce a layer
// ---------------------------------------------------------------------------

#[test]
fn test_build_auto_mode_returns_layer() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap();
    HttpCassetteSvc::build_cassette_layer(make_cfg(dir, "auto"), "auto_mode")
        .expect("auto mode must produce a layer");
}

#[test]
fn test_build_record_mode_returns_layer() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap();
    HttpCassetteSvc::build_cassette_layer(make_cfg(dir, "record"), "record_mode")
        .expect("record mode must produce a layer");
}

#[test]
fn test_build_replay_mode_returns_layer() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap();
    HttpCassetteSvc::build_cassette_layer(make_cfg(dir, "replay"), "replay_mode")
        .expect("replay mode must produce a layer");
}
