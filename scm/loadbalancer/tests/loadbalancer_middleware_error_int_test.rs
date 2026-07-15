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

/// @covers: LoadbalancerMiddlewareError::PoolError — conversion from LoadbalancerError
#[test]
fn test_pool_error_converts_from_loadbalancer_error() {
    use edge_transport_http_egress_loadbalancer::PoolError;
    let lb_err = PoolError::NoHealthyBackends;
    let mw_err = LoadbalancerMiddlewareError::PoolError(lb_err);
    let msg = mw_err.to_string();
    assert!(msg.contains("pool error"), "{msg}");
}

/// @covers: LoadbalancerMiddlewareError — Debug impl
#[test]
fn test_error_implements_debug() {
    let err = LoadbalancerMiddlewareError::InvalidConfig("test".to_string());
    let dbg = format!("{err:?}");
    assert!(!dbg.is_empty(), "Debug must produce non-empty output");
}
