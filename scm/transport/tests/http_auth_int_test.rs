//! Integration tests for `HttpAuth`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::convert::Infallible;
use std::time::Duration;

use bytes::Bytes;
use edge_transport_http_egress_transport::{HttpAuth, HttpConfig, HttpRequest, HttpTransportSvc};
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;

/// @covers: bearer
#[test]
fn test_http_auth_enum_bearer_creates_bearer_auth_with_token() {
    let auth = HttpAuth::bearer("tok_abc");
    assert!(matches!(auth, HttpAuth::Bearer { token } if token == "tok_abc"));
}

/// @covers: basic
#[test]
fn test_http_auth_enum_basic_creates_basic_auth_with_credentials() {
    let auth = HttpAuth::basic("user", "pass");
    assert!(matches!(auth, HttpAuth::Basic { username, .. } if username == "user"));
}

/// @covers: api_key
#[test]
fn test_http_auth_enum_api_key_creates_api_key_auth() {
    let auth = HttpAuth::api_key("X-Api-Key", "secret");
    assert!(matches!(auth, HttpAuth::ApiKey { header, .. } if header == "X-Api-Key"));
}

async fn spawn_echo_header(header_name: &'static str) -> (u16, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let jh = tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            let io = TokioIo::new(stream);
            let _ = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req: Request<Incoming>| async move {
                        let val = req
                            .headers()
                            .get(header_name)
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or("missing")
                            .to_owned();
                        Ok::<_, Infallible>(Response::new(Full::new(Bytes::from(val))))
                    }),
                )
                .await;
        }
    });
    tokio::time::sleep(Duration::from_millis(5)).await;
    (port, jh)
}

/// `HttpRequest::with_auth(HttpAuth::bearer(..))` must produce a real
/// `Authorization: Bearer <token>` header on the wire — proving the
/// constructor's output is actually wired into `send`, not just a
/// standalone value never applied to a request.
#[tokio::test]
async fn test_http_auth_bearer_applies_authorization_header_happy() {
    let (port, _jh) = spawn_echo_header("authorization").await;
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::with_base_url(format!(
        "http://127.0.0.1:{port}"
    )))
    .expect("ok");

    let resp = egress
        .send(HttpRequest::get("/").with_auth(HttpAuth::bearer("tok_abc")))
        .await
        .expect("reachable server must succeed");

    assert_eq!(resp.text().unwrap(), "Bearer tok_abc");
}

/// `HttpAuth::basic` must produce a real `Authorization: Basic <base64>`
/// header — a different credential shape than bearer, proving the wiring
/// dispatches on the auth variant rather than always emitting one fixed header.
#[tokio::test]
async fn test_http_auth_basic_applies_authorization_header_error() {
    let (port, _jh) = spawn_echo_header("authorization").await;
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::with_base_url(format!(
        "http://127.0.0.1:{port}"
    )))
    .expect("ok");

    let resp = egress
        .send(HttpRequest::get("/").with_auth(HttpAuth::basic("alice", "secret")))
        .await
        .expect("reachable server must succeed");

    let body = resp.text().unwrap();
    assert!(
        body.starts_with("Basic "),
        "expected a Basic auth header, got: {body}"
    );
    assert_ne!(
        body, "Bearer tok_abc",
        "basic auth must not collapse into the bearer test's fixed header"
    );
}

/// `HttpAuth::api_key` must apply the credential under the caller-chosen
/// custom header name, not `authorization`.
#[tokio::test]
async fn test_http_auth_api_key_applies_custom_header_edge() {
    let (port, _jh) = spawn_echo_header("x-api-key").await;
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::with_base_url(format!(
        "http://127.0.0.1:{port}"
    )))
    .expect("ok");

    let resp = egress
        .send(HttpRequest::get("/").with_auth(HttpAuth::api_key("x-api-key", "key-value")))
        .await
        .expect("reachable server must succeed");

    assert_eq!(resp.text().unwrap(), "key-value");
}

/// A request built with no `.with_auth(..)` call must not send any
/// `authorization` header — proving `auth: None` is a genuine no-op, not an
/// empty/placeholder credential.
#[tokio::test]
async fn test_http_auth_none_sends_no_authorization_header_edge() {
    let (port, _jh) = spawn_echo_header("authorization").await;
    let egress = HttpTransportSvc::plain_http_egress(HttpConfig::with_base_url(format!(
        "http://127.0.0.1:{port}"
    )))
    .expect("ok");

    let resp = egress
        .send(HttpRequest::get("/"))
        .await
        .expect("reachable server must succeed");

    assert_eq!(resp.text().unwrap(), "missing");
}
