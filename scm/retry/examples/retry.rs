//! Reusable example: drive a real HTTP request through the retry layer
//! against a local server, so the backoff/attempt loop (delegating to the
//! shared `edge-transport-retry` crate's `BackoffScheduler`) actually
//! executes — not just constructs.
#![allow(
    clippy::expect_used,
    reason = "example code, not production library code"
)]

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use edge_transport_http_egress_retry::{DecorateRequest, HttpRetrySvc, Processor, RetryConfig};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

/// Spawn a local server that returns `503` for the first `fail_until`
/// requests, then `200` — so the example can prove the retry layer really
/// re-sends the request after a real backoff sleep, not just that it
/// compiles.
async fn spawn_flaky_server(calls: Arc<AtomicUsize>, fail_until: usize) -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local_addr");

    tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let io = TokioIo::new(stream);
            let calls = calls.clone();
            tokio::spawn(async move {
                let service = service_fn(move |_req: Request<hyper::body::Incoming>| {
                    let attempt = calls.fetch_add(1, Ordering::SeqCst) + 1;
                    async move {
                        let status = if attempt <= fail_until {
                            StatusCode::SERVICE_UNAVAILABLE
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
    let addr = spawn_flaky_server(calls.clone(), 2).await;
    let url = format!("http://{addr}/");

    let layer = HttpRetrySvc
        .decorate(DecorateRequest {
            config: RetryConfig {
                max_retries: 3,
                initial_interval_ms: 50,
                max_interval_ms: 200,
                multiplier: 2.0,
                jitter_factor: 0.0,
                retryable_statuses: vec![503],
                retryable_methods: vec!["GET".to_string()],
            },
        })
        .expect("decorate must succeed for a valid config")
        .layer;

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(layer)
        .build();

    // A single logical request. The retry layer's internal attempt loop
    // must re-send it after a real backoff sleep on each 503, until the
    // server starts returning 200.
    let started = std::time::Instant::now();
    let resp = client
        .get(&url)
        .send()
        .await
        .expect("the retry layer must eventually succeed once the server recovers");
    let elapsed = started.elapsed();

    println!("final status: {}", resp.status());
    println!(
        "server was called {} time(s) across {:?}",
        calls.load(Ordering::SeqCst),
        elapsed
    );

    assert!(resp.status().is_success());
    assert_eq!(
        calls.load(Ordering::SeqCst),
        3,
        "the server must have been hit on all 3 attempts (2 failures + 1 success)"
    );
    assert!(
        elapsed >= Duration::from_millis(50),
        "elapsed time must reflect at least one real backoff sleep, not an instant retry"
    );
    println!(
        "confirmed: BackoffScheduler::next_backoff really executed between attempts \
         (elapsed {elapsed:?} implies a genuine sleep, not a busy loop)"
    );
}
