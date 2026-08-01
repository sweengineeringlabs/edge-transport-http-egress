#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests verifying direct use of the `swe-edge-loadbalancer` dependency.

use swe_edge_loadbalancer::{
    BackendConfig, BackendHealth, BackendId, LoadbalancerConfig, LoadbalancerSvc, Outcome, Strategy,
};

fn two_backend_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig {
                url: "https://api-1.internal".to_string(),
                weight: 1,
            },
            BackendConfig {
                url: "https://api-2.internal".to_string(),
                weight: 1,
            },
        ],
    }
}

/// @covers: LoadbalancerSvc::build_pool — constructs pool from valid config
#[test]
fn test_build_backend_pool_constructs_pool_from_valid_config() {
    let pool = LoadbalancerSvc::build_pool(two_backend_config()).expect("must build");
    assert_eq!(LoadbalancerSvc::backend_count(&pool), 2);
}

/// @covers: LoadbalancerSvc::select — returns a healthy backend
#[test]
fn test_select_backend_returns_healthy_backend() {
    let pool = LoadbalancerSvc::build_pool(two_backend_config()).expect("must build");
    let backend = LoadbalancerSvc::select(&pool).expect("must select");
    assert!(
        !backend.url.is_empty(),
        "selected backend url must not be empty"
    );
    assert_eq!(backend.health, BackendHealth::Healthy);
}

/// @covers: LoadbalancerSvc::report_outcome — failure transitions backend to degraded
#[test]
fn test_report_backend_outcome_failure_transitions_health() {
    let pool = LoadbalancerSvc::build_pool(two_backend_config()).expect("must build");
    let backend = LoadbalancerSvc::select(&pool).expect("must select");
    let id = backend.id.clone();
    LoadbalancerSvc::report_outcome(
        &pool,
        &id,
        Outcome::Failure {
            reason: "500".to_string(),
        },
    );
    // Pool still has healthy backends (second one), so select succeeds.
    let second = LoadbalancerSvc::select(&pool).expect("must select after failure report");
    assert_eq!(second.health, BackendHealth::Healthy);
}

/// @covers: LoadbalancerSvc::report_outcome — success keeps backend healthy
#[test]
fn test_report_backend_outcome_success_keeps_backend_healthy() {
    let pool = LoadbalancerSvc::build_pool(two_backend_config()).expect("must build");
    let backend = LoadbalancerSvc::select(&pool).expect("must select");
    let id = backend.id.clone();
    LoadbalancerSvc::report_outcome(&pool, &id, Outcome::Success);
    let next = LoadbalancerSvc::select(&pool).expect("must still select after success");
    assert_eq!(next.health, BackendHealth::Healthy);
}

/// @covers: BackendId::new — constructs from URL
#[test]
fn test_backend_id_new_stores_url() {
    let id = BackendId::new("https://api.test");
    assert_eq!(id.as_str(), "https://api.test");
}
