//! Integration tests for the `NoopHttpTlsMarker` trait.
//!
//! Rule 120: `src/api/identity/noop_http_tls_marker.rs` requires a
//! corresponding test file.
//!
//! `NoopHttpTlsMarker` is a marker trait for the no-op TLS identity provider.
//! The concrete `NoopHttpTls` type in core/ is `pub(crate)`, so we test the
//! behavior through the public TLS layer API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::{HttpTlsSvc, TlsConfig, TlsLayer};

/// @covers: NoopHttpTlsMarker (via TlsConfig::None layer)
/// Building a `TlsLayer` with `TlsConfig::None` uses the noop TLS provider
/// (which implements `NoopHttpTlsMarker`). The layer must build and report
/// a "noop" provider in its Debug output.
#[test]
fn tls_trait_noop_http_tls_marker_none_layer_debug_contains_noop_int_test() {
    let layer: TlsLayer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("noop"),
        "noop TLS marker layer must report 'noop' in Debug; got: {dbg}"
    );
}

/// @covers: NoopHttpTlsMarker (apply_to pass-through)
/// The noop provider must leave the `ClientBuilder` unmodified — `apply_to`
/// must return `Ok` and the builder must be buildable.
#[test]
fn tls_trait_noop_http_tls_marker_apply_to_is_pass_through_int_test() {
    let layer: TlsLayer = HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("None must build");
    let builder = layer
        .apply_to(reqwest::Client::builder())
        .expect("noop apply_to must succeed");
    let _client = builder
        .build()
        .expect("ClientBuilder must remain buildable after noop apply_to");
}
