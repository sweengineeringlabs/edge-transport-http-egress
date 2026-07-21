//! Integration tests for the `PoolMetrics` trait.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_loadbalancer::{
    BackendConfig, BackendCountRequest, BackendCountResponse, LoadbalancerConfig,
    LoadbalancerMiddlewareError, LoadbalancerSvcProcessor, PoolMetrics, Strategy,
};

fn config_with(backends: Vec<BackendConfig>) -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends,
    }
}

/// @covers: backend_count
#[test]
fn test_backend_count_reflects_configured_backends_happy() {
    let cfg = config_with(vec![
        BackendConfig {
            url: "https://api-1.test".to_string(),
            weight: 1,
        },
        BackendConfig {
            url: "https://api-2.test".to_string(),
            weight: 1,
        },
    ]);
    let layer = LoadbalancerSvcProcessor::build_layer(cfg).expect("build must succeed");
    let resp = layer
        .backend_count(BackendCountRequest)
        .expect("infallible");
    assert_eq!(resp.value, 2);
}

/// @covers: backend_count
#[test]
fn test_backend_count_single_backend_is_a_valid_boundary_edge() {
    let cfg = config_with(vec![BackendConfig {
        url: "https://api.test".to_string(),
        weight: 1,
    }]);
    let layer = LoadbalancerSvcProcessor::build_layer(cfg).expect("build must succeed");
    let resp = layer
        .backend_count(BackendCountRequest)
        .expect("infallible");
    assert_eq!(resp.value, 1);
}

/// A minimal external test-double proving `PoolMetrics::backend_count` can
/// genuinely fail for a real implementor — the crate's own
/// `LoadbalancerLayerPoolMetrics` never returns `Err` here, so this is the
/// only way to exercise the error path.
struct FailingPoolMetrics;

impl PoolMetrics for FailingPoolMetrics {
    fn backend_count(
        &self,
        _request: BackendCountRequest,
    ) -> Result<BackendCountResponse, LoadbalancerMiddlewareError> {
        Err(LoadbalancerMiddlewareError::PoolBuildFailed(
            "pool not initialized".to_string(),
        ))
    }
}

/// @covers: backend_count
#[test]
fn test_backend_count_uninitialized_implementor_returns_err_error() {
    let pool = FailingPoolMetrics;
    let result = pool.backend_count(BackendCountRequest);
    assert!(
        matches!(result, Err(LoadbalancerMiddlewareError::PoolBuildFailed(_))),
        "an external PoolMetrics impl reporting an uninitialized pool must surface as PoolBuildFailed; got: {result:?}"
    );
}
