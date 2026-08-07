//! Reusable example: drive real HTTP requests through the breaker layer
//! against a local server, so the circuit-breaker state machine
//! (`DefaultBreakerTransition`, moved to the shared `edge-transport-breaker`
//! crate) actually executes — not just constructs.
#![allow(
    clippy::expect_used,
    reason = "example code, not production library code"
)]

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use edge_transport_http_egress_breaker::{BreakerConfig, HttpBreakerSvcProcessor};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

/// Spawn a local server whose response status is controllable from outside,
/// and whose request count is observable — so the example can prove the
/// breaker really rejects requests without reaching the server once open.
async fn spawn_controllable_server(
    calls: Arc<AtomicUsize>,
    should_fail: Arc<AtomicBool>,
) -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local_addr");

    tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let io = TokioIo::new(stream);
            let calls = calls.clone();
            let should_fail = should_fail.clone();
            tokio::spawn(async move {
                let service = service_fn(move |_req: Request<hyper::body::Incoming>| {
                    calls.fetch_add(1, Ordering::SeqCst);
                    let fail = should_fail.load(Ordering::SeqCst);
                    async move {
                        let status = if fail {
                            StatusCode::INTERNAL_SERVER_ERROR
                        } else {
                            StatusCode::OK
                        };
                        let resp = Response::builder()
                            .status(status)
                            .body(Full::new(Bytes::from_static(b"ok")))
                            .expect("response must build");
                        Ok::<_, Infallible>(resp)
                    }
                });
                let _ = http1::Builder::new().serve_connection(io, service).await;
            });
        }
    });

    addr
}

#[tokio::main]
async fn main() {
    let calls = Arc::new(AtomicUsize::new(0));
    let should_fail = Arc::new(AtomicBool::new(true));
    let addr = spawn_controllable_server(calls.clone(), should_fail.clone()).await;
    let url = format!("http://{addr}/");

    let layer = HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig {
        failure_threshold: 2,
        half_open_after_seconds: 1,
        reset_after_successes: 1,
        failure_statuses: vec![500],
    })
    .expect("build_breaker_layer must succeed for a valid config");

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(layer)
        .build();

    // Two real 500 responses reach the server and trip the breaker.
    for attempt in 1..=2 {
        let resp = client.get(&url).send().await;
        println!(
            "attempt {attempt}: {:?} (server calls so far: {})",
            resp.map(|r| r.status()),
            calls.load(Ordering::SeqCst)
        );
    }

    // Third call: the circuit is now open. This is the real proof that
    // `DefaultBreakerTransition::admit` executed and changed state — the
    // request is rejected WITHOUT reaching the server (call count unchanged).
    let before = calls.load(Ordering::SeqCst);
    let err = client
        .get(&url)
        .send()
        .await
        .expect_err("an open circuit must reject the request");
    let after = calls.load(Ordering::SeqCst);
    println!("attempt 3 (circuit open): {err}");
    assert!(
        err.to_string().contains("circuit open"),
        "rejection must come from BreakerError::CircuitOpen, got: {err}"
    );
    assert_eq!(
        before, after,
        "an open circuit must reject without reaching the server"
    );
    println!("confirmed: circuit rejected the request without reaching the server ({after} total server calls)");

    // Wait out the cool-down, let the server recover, and prove the breaker
    // really re-admits a half-open probe and closes again.
    tokio::time::sleep(Duration::from_secs(1)).await;
    should_fail.store(false, Ordering::SeqCst);
    let resp = client
        .get(&url)
        .send()
        .await
        .expect("the half-open probe must reach the server and succeed");
    println!(
        "attempt 4 (after cool-down, server recovered): {}",
        resp.status()
    );
    assert!(resp.status().is_success());
    println!("confirmed: breaker transitioned Open -> HalfOpen -> Closed for real");
}
