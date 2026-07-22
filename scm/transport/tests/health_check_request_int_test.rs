//! Integration tests for `HealthCheckRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::convert::Infallible;
use std::time::Duration;

use bytes::Bytes;
use edge_transport_http_egress_transport::{HealthCheckRequest, HttpConfig, HttpTransportSvc};
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
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

/// A reachable, 2xx-responding base URL must pass `health_check`.
#[tokio::test]
async fn test_health_check_request_reachable_server_returns_ok_happy() {
    let (port, _jh) = spawn_once(|_req| async { Response::new(Full::new(Bytes::new())) }).await;

    let cfg = HttpConfig::with_base_url(format!("http://127.0.0.1:{port}"));
    let egress = HttpTransportSvc::plain_http_egress(cfg).expect("ok");

    let result = egress.health_check(HealthCheckRequest).await;
    assert!(
        result.is_ok(),
        "a 2xx-responding server must pass health_check"
    );
}

/// A reachable but 5xx-responding base URL must fail `health_check`, proving
/// the check inspects the actual status rather than only connectivity.
#[tokio::test]
async fn test_health_check_request_server_error_status_returns_err_error() {
    let (port, _jh) = spawn_once(|_req| async {
        let mut resp = Response::new(Full::new(Bytes::new()));
        *resp.status_mut() = StatusCode::SERVICE_UNAVAILABLE;
        resp
    })
    .await;

    let cfg = HttpConfig::with_base_url(format!("http://127.0.0.1:{port}"));
    let egress = HttpTransportSvc::plain_http_egress(cfg).expect("ok");

    let result = egress.health_check(HealthCheckRequest).await;
    assert!(
        result.is_err(),
        "a 503-responding server must fail health_check, not just 'connected == healthy'"
    );
}
