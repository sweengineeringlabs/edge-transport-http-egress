//! Integration tests for `edge_transport_http_egress_cassette` trait re-exports (`api/traits.rs`).
//!
//! `api/traits.rs` declares a `pub(crate)` type alias `HttpCassetteTrait`
//! for `dyn HttpCassette`. That alias is internal-only. From the integration
//! surface the relevant contract is: the SAF re-export surface is complete
//! and the `CassetteLayer` produced by the builder satisfies all trait bounds
//! required for use inside `reqwest_middleware::ClientBuilder`.
//!
//! These tests confirm that the full middleware trait chain compiles — if
//! `reqwest_middleware::Middleware` is removed from `CassetteLayer`'s impl
//! set, or if the trait object indirection through `HttpCassetteTrait` is
//! broken, consumers of the crate would fail to compile.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{CassetteConfig, HttpCassetteSvc};

// ---------------------------------------------------------------------------
// reqwest_middleware::Middleware — CassetteLayer must implement it
// ---------------------------------------------------------------------------

/// `CassetteLayer` must implement `reqwest_middleware::Middleware` so it can be
/// attached via `.with(layer)`. Proven by actually driving the middleware: in
/// replay mode with no recorded fixture, a request must fail loudly with the
/// crate's "no recorded interaction" error — exercising `Middleware::handle`
/// end-to-end without touching the network.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_cassette_layer_implements_reqwest_middleware() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "replay".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string(), "url".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    let layer = HttpCassetteSvc::build_cassette_layer(cfg, "middleware_replay_miss")
        .expect("build must succeed");
    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(layer)
        .build();
    let err = client
        .get("https://api.example.test/never-recorded")
        .send()
        .await
        .expect_err("replay-mode cache miss must fail loudly, not hit the network");
    assert!(
        err.to_string().contains("no recorded interaction"),
        "replay miss must produce the crate's cache-miss error; got: {err}"
    );
}

// ---------------------------------------------------------------------------
// CassetteLayer is usable in an Arc<T> context (trait-object indirection)
// ---------------------------------------------------------------------------

/// `CassetteLayer` must be `Send + Sync` so it can be wrapped in `Arc<T>` and
/// shared across executor threads. Proven by cloning an `Arc<CassetteLayer>`
/// into a second OS thread and asserting both handles observe the same layer.
#[test]
fn test_cassette_layer_is_arc_send_sync() {
    use std::sync::Arc;
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "auto".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    let layer =
        HttpCassetteSvc::build_cassette_layer(cfg, "arc_send_sync").expect("build must succeed");
    let shared = Arc::new(layer);
    let clone = Arc::clone(&shared);
    let other = std::thread::spawn(move || format!("{clone:?}"))
        .join()
        .expect("thread owning the Arc clone must not panic");
    assert_eq!(
        other,
        format!("{shared:?}"),
        "both Arc handles must observe the same layer state across threads"
    );
}

// ---------------------------------------------------------------------------
// Full pipeline: layer can be passed to reqwest_middleware::ClientBuilder
// ---------------------------------------------------------------------------

/// Building a `reqwest_middleware::ClientWithMiddleware` with a
/// `CassetteLayer` must compile and construct without panic. This test
/// does not make a real HTTP request; it validates that the type chain
/// from `CassetteLayer` → `reqwest_middleware::Middleware` → `ClientBuilder`
/// is fully wired.
#[test]
fn test_cassette_layer_attaches_to_reqwest_middleware_client_builder() {
    let tmpdir = tempfile::tempdir().unwrap();
    let dir = tmpdir.path().to_str().unwrap().replace('\\', "/");
    let cfg = CassetteConfig {
        mode: "replay".to_string(),
        cassette_dir: dir,
        match_on: vec!["method".to_string(), "url".to_string()],
        scrub_headers: vec![],
        scrub_body_paths: vec![],
    };
    let layer = HttpCassetteSvc::build_cassette_layer(cfg, "trait_chain_check")
        .expect("build must succeed");

    let _client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(layer)
        .build();
    // No assertion on `_client`'s behavior — the test proves the type chain
    // compiles and the constructor succeeds.
}
