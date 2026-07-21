#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `LoadbalancerMiddlewareError`.

use edge_transport_http_egress_loadbalancer::LoadbalancerMiddlewareError;

/// @covers: LoadbalancerMiddlewareError::InvalidConfig — display message
#[test]
fn test_invalid_config_displays_reason() {
    let err = LoadbalancerMiddlewareError::InvalidConfig("backends must not be empty".to_string());
    let msg = err.to_string();
    assert!(msg.contains("invalid configuration"), "{msg}");
    assert!(msg.contains("backends must not be empty"), "{msg}");
}

/// @covers: LoadbalancerMiddlewareError::InvalidBackendUrl — display message
#[test]
fn test_invalid_backend_url_displays_reason() {
    let err = LoadbalancerMiddlewareError::InvalidBackendUrl("not a url".to_string());
    let msg = err.to_string();
    assert!(msg.contains("invalid backend URL"), "{msg}");
}

/// @covers: LoadbalancerMiddlewareError::PoolBuildFailed — display message
#[test]
fn test_pool_build_failed_displays_reason() {
    let err = LoadbalancerMiddlewareError::PoolBuildFailed("no healthy backends".to_string());
    let msg = err.to_string();
    assert!(msg.contains("pool error"), "{msg}");
    assert!(msg.contains("no healthy backends"), "{msg}");
}

/// @covers: LoadbalancerMiddlewareError — Debug impl
#[test]
fn test_error_implements_debug() {
    let err = LoadbalancerMiddlewareError::InvalidConfig("test".to_string());
    let dbg = format!("{err:?}");
    assert!(!dbg.is_empty(), "Debug must produce non-empty output");
}
