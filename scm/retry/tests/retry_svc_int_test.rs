//! Anchor tests for `saf/retry/retry_svc.rs` — SEA Rule 220 compliance.

#![allow(clippy::unwrap_used)]

use edge_transport_http_egress_retry::{HttpRetrySvc, RetryConfig};

#[test]
fn test_create_config_builder_returns_named_builder_happy() {
    let b = HttpRetrySvc::create_config_builder();
    assert!(!b.name().is_empty());
}

#[test]
fn test_build_retry_layer_default_config_succeeds_happy() {
    assert!(HttpRetrySvc::build_retry_layer(RetryConfig::default()).is_ok());
}

#[test]
fn test_build_retry_layer_called_twice_both_succeed_edge() {
    let r1 = HttpRetrySvc::build_retry_layer(RetryConfig::default());
    let r2 = HttpRetrySvc::build_retry_layer(RetryConfig::default());
    assert!(r1.is_ok() && r2.is_ok());
}
