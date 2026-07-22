//! Dependency coverage test for `edge-transport-http-egress-retry`.
//! @covers: edge-transport-http-egress-retry
//!
//! Rule 95: `edge-transport-http-egress-retry` is used in `src/` (feature =
//! "retry", enabled by default) and must have integration coverage with an
//! explicit `use edge_transport_http_egress_retry::...` import.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::{DecorateRequest, HttpRetrySvc, Processor, RetryConfig};

/// @covers: edge-transport-http-egress-retry
/// Verifies the `edge_transport_http_egress_retry` crate is accessible and
/// that its `Processor::decorate` entry point (the one `transport_svc.rs`
/// dispatches through when the `retry` feature is enabled) genuinely applies
/// the supplied policy rather than a hardcoded default.
#[test]
fn transport_struct_dep_edge_transport_http_egress_retry_decorate_int_test() {
    let cfg = RetryConfig {
        max_retries: 9,
        ..RetryConfig::default()
    };
    let resp = HttpRetrySvc
        .decorate(DecorateRequest { config: cfg })
        .expect("valid retry config must build a layer");
    assert!(
        format!("{:?}", resp.layer).contains("max_retries: 9"),
        "the retry layer built via the dependency's Processor trait must carry the configured policy"
    );
}
