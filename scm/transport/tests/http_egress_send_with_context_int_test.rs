//! Integration tests for `HttpEgress::send_with_context`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;
use std::convert::Infallible;
use std::time::Duration;

use bytes::Bytes;
use edge_transport_http_egress_transport::{
    HttpConfig, HttpRequest, HttpTransportSvc, SecurityContext,
};
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;

// ─── test-server helper ──────────────────────────────────────────────────────

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

// ─── tests ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_send_with_context_delegates_to_send_and_returns_same_response() {
    // Arrange: a server that echoes the request method back in the body.
    let (port, _jh) =
        spawn_once(|_req| async { Response::new(Full::new(Bytes::from("hello-from-server"))) })
            .await;

    let cfg = HttpConfig::with_base_url(format!("http://127.0.0.1:{port}"));
    let client = HttpTransportSvc::plain_http_egress(cfg).unwrap();

    let ctx = SecurityContext {
        principal: None,
        tenant_id: Some("tenant-42".to_string()),
        claims: HashMap::new(),
        trace_id: Some("trace-abc".to_string()),
        authenticated: true,
        token: None,
        metadata: HashMap::new(),
        is_authorized: false,
        extensions: HashMap::new(),
    };

    // Act: call send_with_context — default impl must delegate to send.
    let resp = client
        .send_with_context(HttpRequest::get("/"), ctx.into())
        .await
        .expect("send_with_context must succeed when server is reachable");

    // Assert: the response body is exactly what the server returned.
    assert!(
        resp.is_success(),
        "expected 2xx status, got {}",
        resp.status
    );
    assert_eq!(
        resp.body, b"hello-from-server",
        "send_with_context must return the same body as send"
    );
}

#[tokio::test]
async fn test_send_with_context_propagates_request_unchanged_to_server() {
    // Arrange: server echoes the x-custom header so we can verify the
    // request reached the server unmodified.
    let (port, _jh) = spawn_once(|req| async move {
        let val = req
            .headers()
            .get("x-custom")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("missing")
            .to_owned();
        Response::new(Full::new(Bytes::from(val)))
    })
    .await;

    let cfg = HttpConfig::with_base_url(format!("http://127.0.0.1:{port}"));
    let client = HttpTransportSvc::plain_http_egress(cfg).unwrap();

    let mut request = HttpRequest::get("/");
    request
        .headers
        .insert("x-custom".to_string(), "my-value".to_string());

    let ctx = SecurityContext {
        principal: None,
        tenant_id: None,
        claims: HashMap::new(),
        trace_id: None,
        authenticated: false,
        token: None,
        metadata: HashMap::new(),
        is_authorized: false,
        extensions: HashMap::new(),
    };

    // Act
    let resp = client
        .send_with_context(request, ctx.into())
        .await
        .expect("send_with_context must succeed");

    // Assert: the request-level header arrived at the server.
    let echoed = String::from_utf8(resp.body).unwrap();
    assert_eq!(
        echoed, "my-value",
        "send_with_context must forward request headers to the server"
    );
}
