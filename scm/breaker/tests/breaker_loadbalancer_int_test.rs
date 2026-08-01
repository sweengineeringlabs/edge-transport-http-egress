//! Integration tests for the `loadbalancer` feature of `edge-transport-http-egress-breaker`.
//!
//! Verifies that `BreakerLayerBreakerMetrics` reports circuit-trip and recovery events back
//! to the attached `BackendPoolInstance`, keeping pool health in sync with the
//! circuit state.

#![cfg(feature = "loadbalancer")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::Empty;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use reqwest_middleware::{Middleware, Next};
use tokio::net::TcpListener;

use edge_transport_http_egress_breaker::{BreakerConfig, HttpBreakerSvcProcessor};
use swe_edge_loadbalancer::{
    BackendConfig, BackendId, BackendPoolInstance, EgressError, LoadbalancerConfig,
    LoadbalancerError, LoadbalancerSvc, Strategy,
};

// ---------------------------------------------------------------------------
// Test server
// ---------------------------------------------------------------------------

/// Spawn a minimal hyper HTTP server on an OS-assigned port.
///
/// While `fail` is `true` the server returns 500; when `false` it returns 200.
/// Returns the bound port number.
async fn spawn_test_server(fail: Arc<AtomicBool>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().expect("local_addr").port();

    tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let flag = Arc::clone(&fail);
            tokio::spawn(async move {
                let svc = service_fn(move |_req: hyper::Request<Incoming>| {
                    let flag2 = Arc::clone(&flag);
                    async move {
                        let status = if flag2.load(Ordering::SeqCst) {
                            500u16
                        } else {
                            200u16
                        };
                        Ok::<_, hyper::Error>(
                            hyper::Response::builder()
                                .status(status)
                                .body(Empty::<Bytes>::new())
                                .unwrap(),
                        )
                    }
                });
                http1::Builder::new()
                    .serve_connection(TokioIo::new(stream), svc)
                    .await
                    .ok();
            });
        }
    });

    port
}

// ---------------------------------------------------------------------------
// Pool factory
// ---------------------------------------------------------------------------

fn make_pool(url: &str) -> Arc<BackendPoolInstance> {
    let cfg = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: url.to_string(),
            weight: 1,
        }],
    };
    Arc::new(LoadbalancerSvc::build_pool(cfg).expect("build_pool"))
}

// ---------------------------------------------------------------------------
// BackendId injector — simulates LoadbalancerLayer
// ---------------------------------------------------------------------------

/// Middleware that injects a `BackendId` into `ext` before calling the next
/// layer. Simulates what `LoadbalancerLayer` does in production so the outer
/// `BreakerLayerBreakerMetrics` can read it after `next.run()` returns.
struct BackendIdInjector {
    id: BackendId,
}

impl BackendIdInjector {
    fn new(url: &str) -> Self {
        Self {
            id: BackendId::new(url),
        }
    }
}

#[async_trait]
impl Middleware for BackendIdInjector {
    async fn handle(
        &self,
        req: reqwest::Request,
        ext: &mut http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        ext.insert(self.id.clone());
        next.run(req, ext).await
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn breaker_config(failure_threshold: u32, half_open_secs: u64) -> BreakerConfig {
    BreakerConfig {
        failure_threshold,
        half_open_after_seconds: half_open_secs,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// When the breaker trips (failure_threshold consecutive 500s), `BreakerLayerBreakerMetrics`
/// must report `Outcome::Failure` to the pool, degrading the backend so that
/// `LoadbalancerSvc::select` returns `NoHealthyBackends`.
///
/// @covers: build_breaker_layer_with_pool
#[tokio::test]
async fn test_breaker_loadbalancer_trip_degrades_backend_in_pool() {
    let fail_flag = Arc::new(AtomicBool::new(true)); // always 500
    let port = spawn_test_server(Arc::clone(&fail_flag)).await;
    let url = format!("http://127.0.0.1:{port}/");

    let pool = make_pool(&url);
    let layer = HttpBreakerSvcProcessor::build_breaker_layer_with_pool(
        breaker_config(2, 60),
        Arc::clone(&pool),
    )
    .expect("build layer");

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(layer)
        .with(BackendIdInjector::new(&url))
        .build();

    // Two consecutive 500s must reach the failure threshold and open the circuit.
    for _ in 0..2 {
        let _ = client.get(&url).send().await;
    }

    let err = LoadbalancerSvc::select(&pool).unwrap_err();
    assert!(
        matches!(
            err,
            LoadbalancerError::Egress(EgressError::NoHealthyBackends)
        ),
        "pool must have no healthy backends after breaker trip; got: {err:?}"
    );
}

/// After the circuit opens, the next request transitions it to HalfOpen.
/// A successful probe (200) must close the circuit and report `Outcome::Success`
/// to the pool, restoring the backend to Healthy.
#[tokio::test]
async fn test_breaker_loadbalancer_recovery_restores_backend_in_pool() {
    let fail_flag = Arc::new(AtomicBool::new(true)); // start 500
    let port = spawn_test_server(Arc::clone(&fail_flag)).await;
    let url = format!("http://127.0.0.1:{port}/");

    let pool = make_pool(&url);
    let layer = HttpBreakerSvcProcessor::build_breaker_layer_with_pool(
        // half_open_after_seconds=0 → circuit transitions to HalfOpen on the
        // very next admit() call, no sleep needed.
        breaker_config(2, 0),
        Arc::clone(&pool),
    )
    .expect("build layer");

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(layer)
        .with(BackendIdInjector::new(&url))
        .build();

    // Trip the circuit.
    for _ in 0..2 {
        let _ = client.get(&url).send().await;
    }
    assert!(
        matches!(
            LoadbalancerSvc::select(&pool).unwrap_err(),
            LoadbalancerError::Egress(EgressError::NoHealthyBackends)
        ),
        "pool must be degraded after trip"
    );

    // Switch the server to 200. The next request goes through as a HalfOpen
    // probe. One success closes the circuit (reset_after_successes=1) and
    // reports Success to the pool.
    fail_flag.store(false, Ordering::SeqCst);
    let _ = client.get(&url).send().await;

    let backend = LoadbalancerSvc::select(&pool)
        .expect("backend must be Healthy after successful half-open probe");
    assert_eq!(
        backend.id,
        BackendId::new(&url),
        "recovered backend id must match the registered backend"
    );
}
