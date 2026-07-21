//! Integration tests for `GetRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::convert::Infallible;
use std::time::Duration;

use bytes::Bytes;
use edge_transport_http_egress_transport::{GetRequest, HttpConfig, HttpTransportSvc};
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

/// `GetRequest.url` drives the request path — the response body must reflect
/// the path the server actually received.
#[tokio::test]
async fn test_get_request_url_field_reaches_server_happy() {
    let (port, _jh) = spawn_once(|req| async move {
        Response::new(Full::new(Bytes::from(req.uri().path().to_string())))
    })
    .await;

    let cfg = HttpConfig::with_base_url(format!("http://127.0.0.1:{port}"));
    let egress = HttpTransportSvc::plain_http_egress(cfg).expect("ok");

    let resp = egress
        .get(GetRequest {
            url: "/widgets/42".to_string(),
        })
        .await
        .expect("reachable server must succeed");

    assert_eq!(
        resp.text().unwrap(),
        "/widgets/42",
        "GetRequest.url must be the exact path forwarded to the server"
    );
}

/// An unreachable URL must surface as an `Err`, proving `get` isn't a stub
/// that always succeeds regardless of `GetRequest.url`.
#[tokio::test]
async fn test_get_request_unreachable_url_returns_err_error() {
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("ok");
    let result = egress
        .get(GetRequest {
            url: "http://0.0.0.0:1/unreachable".to_string(),
        })
        .await;
    assert!(result.is_err());
}
