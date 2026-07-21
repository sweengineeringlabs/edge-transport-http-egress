//! End-to-end integration tests for the SWE-builtin egress
//! (`HttpTransportSvc::default_http_egress_with_config`, `core/default`).
//!
//! `default_http_egress_int_test.rs` and `coverage_int_test.rs` only assert
//! that the builtin factory *builds* successfully; neither drives a real
//! request through the result. These tests close that gap by sending against
//! a real local server.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::convert::Infallible;
use std::time::Duration;

use bytes::Bytes;
use edge_transport_http_egress_transport::{HttpConfig, HttpRequest, HttpTransportSvc};
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;

async fn spawn_once<F, Fut>(handler: F) -> (u16, tokio::task::JoinHandle<()>)
where
    F: Fn(Request<Incoming>) -> Fut + Send + Clone + 'static,
    Fut: std::future::Future<Output = Response<Full<Bytes>>> + Send,
{
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let jh = tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            let io = TokioIo::new(stream);
            let _ = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req: Request<Incoming>| {
                        let handler = handler.clone();
                        async move { Ok::<_, Infallible>(handler(req).await) }
                    }),
                )
                .await;
        }
    });
    tokio::time::sleep(Duration::from_millis(5)).await;
    (port, jh)
}

/// The SWE-builtin egress must actually deliver a request to a real server
/// and return its body, not just build without error.
#[tokio::test]
async fn test_http_egress_builtin_sends_real_request_happy() {
    let (port, _jh) =
        spawn_once(|_req| async { Response::new(Full::new(Bytes::from("builtin-ok"))) }).await;

    let egress = HttpTransportSvc::default_http_egress_with_config(HttpConfig::with_base_url(
        format!("http://127.0.0.1:{port}"),
    ))
    .expect("builtin egress must build");

    let resp = egress
        .send(HttpRequest::get("/"))
        .await
        .expect("builtin egress must reach a real server");

    assert_eq!(resp.text().unwrap(), "builtin-ok");
}

/// An unreachable base URL through the same builtin factory must fail the
/// send — proving the builtin egress isn't a stub that always succeeds.
#[tokio::test]
async fn test_http_egress_builtin_unreachable_server_returns_err_error() {
    let egress = HttpTransportSvc::default_http_egress_with_config(HttpConfig::with_base_url(
        "http://0.0.0.0:1",
    ))
    .expect("builtin egress must build even with an unreachable base_url");

    let result = egress.send(HttpRequest::get("/")).await;
    assert!(result.is_err());
}
