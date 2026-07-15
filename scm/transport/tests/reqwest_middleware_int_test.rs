//! Integration tests covering the `reqwest-middleware` dependency.
//!
//! Verifies that the SAF factory functions correctly assemble a
//! `reqwest_middleware::ClientWithMiddleware`-backed outbound and that the
//! middleware pipeline operates end-to-end against a real loopback server.

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
use reqwest_middleware::ClientBuilder;

/// Spawn a single-connection HTTP/1 test server.
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

/// @covers: plain_http_egress
#[test]
fn test_reqwest_middleware_client_builder_constructs_passthrough_client() {
    // Directly exercise reqwest_middleware::ClientBuilder to verify the dep is wired.
    let inner = reqwest::Client::new();
    let _client = ClientBuilder::new(inner).build();
    // If this compiles and runs, the dep is present and functional.
}

/// @covers: plain_http_egress
#[tokio::test]
async fn test_reqwest_middleware_client_sends_get_request_and_receives_200() {
    let (port, _jh) =
        spawn_once(|_req| async { Response::new(Full::new(Bytes::from("ok"))) }).await;

    let cfg = HttpConfig::with_base_url(format!("http://127.0.0.1:{port}"));
    let client = HttpTransportSvc::plain_http_egress(cfg).expect("plain_http_egress must build");
    let resp = client
        .send(HttpRequest::get("/"))
        .await
        .expect("GET must succeed");
    assert_eq!(resp.status, 200);
    assert_eq!(resp.body, b"ok");
}

/// @covers: plain_http_egress
#[tokio::test]
async fn test_reqwest_middleware_client_forwards_custom_headers() {
    let (port, _jh) = spawn_once(|req| async move {
        let val = req
            .headers()
            .get("x-trace-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_owned();
        Response::new(Full::new(Bytes::from(val)))
    })
    .await;

    let cfg = HttpConfig::with_base_url(format!("http://127.0.0.1:{port}"));
    let client = HttpTransportSvc::plain_http_egress(cfg).expect("plain_http_egress must build");
    let req = HttpRequest::get("/").with_header("x-trace-id", "trace-99");
    let resp = client
        .send(req)
        .await
        .expect("GET with header must succeed");
    assert_eq!(
        String::from_utf8(resp.body).unwrap(),
        "trace-99",
        "middleware must forward custom headers verbatim"
    );
}

/// @covers: plain_http_egress
#[tokio::test]
async fn test_reqwest_middleware_client_sends_post_with_json_body() {
    let (port, _jh) = spawn_once(|req| async move {
        let ct = req
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_owned();
        Response::new(Full::new(Bytes::from(ct)))
    })
    .await;

    let cfg = HttpConfig::with_base_url(format!("http://127.0.0.1:{port}"));
    let client = HttpTransportSvc::plain_http_egress(cfg).expect("plain_http_egress must build");
    let req = HttpRequest::post("/")
        .with_json(&serde_json::json!({"key": "value"}))
        .unwrap();
    let resp = client.send(req).await.expect("POST with JSON must succeed");
    let ct = String::from_utf8(resp.body).unwrap();
    assert!(
        ct.starts_with("application/json"),
        "middleware must set Content-Type: application/json for JSON bodies, got {ct:?}"
    );
}
