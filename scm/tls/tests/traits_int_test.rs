//! Integration tests for `edge_transport_http_egress_tls` trait re-exports (`api/traits.rs`).
//!
//! `api/traits.rs` defines the `pub(crate)` type alias `HttpTlsTrait` for
//! `dyn HttpTls`. The integration-level contract is:
//!
//! - The SAF re-export surface is complete: `TlsConfig`, `TlsLayer`,
//!   `TlsConfigError`, and `HttpTlsSvc::build_tls_layer()` are all accessible.
//! - `TlsLayer::apply_to` works end-to-end with a `reqwest::ClientBuilder`.
//! - `TlsLayer` is `Send + Sync` (flows from `HttpTls: Send + Sync + Debug`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::{HttpTlsSvc, TlsConfig, TlsConfigError, TlsLayer};

// ---------------------------------------------------------------------------
// SAF re-export completeness — compile-time proof
// ---------------------------------------------------------------------------

/// All required public items exported by the SAF surface must be reachable from
/// the crate root. A missing re-export causes this test to fail to compile.
#[test]
fn test_saf_surface_exports_all_required_types() {
    // TlsConfig — type
    let _ = TlsConfig::None;

    // TlsLayer — type
    fn accept_layer(_: TlsLayer) {}
    let _ = accept_layer as fn(TlsLayer);

    // TlsConfigError — type
    let _e = TlsConfigError::Config("test".to_string());
}

// ---------------------------------------------------------------------------
// HttpTls trait object safety — Arc<dyn HttpTls> must compile
// ---------------------------------------------------------------------------

/// `HttpTls` must be object-safe (Send + Sync + Debug supertrait bounds
/// are satisfied). Although `HttpTls` itself is `pub(crate)`, its effect
/// propagates to `TlsLayer` which holds `Arc<dyn HttpTls>`. If the trait
/// loses object-safety this test fails to compile.
#[test]
fn test_tls_layer_holds_arc_dyn_provider() {
    // The TlsLayer itself proves Arc<dyn HttpTls> works.
    let layer: TlsLayer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    // If Arc<dyn HttpTls> weren't working, build() would fail to compile.
    drop(layer);
}

// ---------------------------------------------------------------------------
// Send + Sync — required by HttpTls supertrait bounds
// ---------------------------------------------------------------------------

#[test]
fn test_tls_layer_satisfies_send_sync_from_http_tls_supertraits() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TlsLayer>();
}

// ---------------------------------------------------------------------------
// End-to-end pipeline: build_tls_layer → layer → apply_to → client
// ---------------------------------------------------------------------------

/// The full pipeline through the SAF surface must compile and run without
/// panic for the `TlsConfig::None` case.
#[test]
fn test_full_saf_pipeline_none_config_builds_client() {
    let layer: TlsLayer =
        HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None config must build");
    let cb = layer
        .apply_to(reqwest::Client::builder())
        .expect("apply_to must succeed");
    let _client = cb.build().expect("ClientBuilder must build");
}

/// The full pipeline through `HttpTlsSvc::build_tls_layer(TlsConfig::None)` must
/// also produce a working client.
#[test]
fn test_full_with_config_pipeline_none_config_builds_client() {
    let layer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    let _client = layer
        .apply_to(reqwest::Client::builder())
        .expect("apply_to")
        .build()
        .expect("build client");
}
