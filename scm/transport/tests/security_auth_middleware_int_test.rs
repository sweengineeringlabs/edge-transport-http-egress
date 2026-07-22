//! Integration tests for the auth-strategy middleware layer wired by
//! `HttpTransportSvc::http_egress_from_config_with_auth` (`core/auth`).
//!
//! Unlike the build-only coverage in `http_egress_from_config_int_test.rs`,
//! these tests actually send a request through the assembled egress and
//! observe, at a real local server, that the supplied
//! `HttpEgressAuthStrategy::authorize` call genuinely mutates every outbound
//! request rather than only being wired without effect.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use edge_security_transport_egress_http::{
    AuthorizeRequest, AuthorizeResponse, HttpEgressAuthError, HttpEgressAuthStrategy,
    OutboundHttpRequest,
};
use edge_transport_http_egress_transport::{HttpRequest, HttpTransportSvc};
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use reqwest::header::{HeaderName, HeaderValue};
use swe_edge_configbuilder::ConfigLoaderFactory;
use tempfile::TempDir;

fn loader(content: &str) -> (TempDir, swe_edge_configbuilder::SectionLoaderImpl) {
    let dir = TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), content).expect("write application.toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
}

/// A strategy that stamps every outbound request with a fixed bearer header.
struct StampingAuthStrategy;

impl HttpEgressAuthStrategy for StampingAuthStrategy {
    fn authorize(
        &self,
        request: AuthorizeRequest,
    ) -> Result<AuthorizeResponse, HttpEgressAuthError> {
        let mut inner = request.request.into_inner();
        inner.headers_mut().insert(
            HeaderName::from_static("authorization"),
            HeaderValue::from_static("Bearer stamped-token"),
        );
        Ok(AuthorizeResponse {
            request: OutboundHttpRequest::new(inner),
        })
    }
}

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

/// `SecurityAuthMiddleware` must invoke the supplied strategy on every real
/// outbound request, and the mutated (stamped) request must be what actually
/// reaches the server.
#[tokio::test]
async fn test_security_auth_middleware_stamps_every_outbound_request_happy() {
    let (port, _jh) = spawn_once(|req| async move {
        let auth = req
            .headers()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("missing")
            .to_owned();
        Response::new(Full::new(Bytes::from(auth)))
    })
    .await;

    let (_d, l) = loader("[unrelated]\nx = 1");
    let strategy: Arc<dyn HttpEgressAuthStrategy> = Arc::new(StampingAuthStrategy);
    let egress = HttpTransportSvc::http_egress_from_config_with_auth(&l, strategy)
        .expect("auth-only egress must build");

    let resp = egress
        .send(HttpRequest::get(format!("http://127.0.0.1:{port}/")))
        .await
        .expect("reachable server must succeed");

    assert_eq!(
        resp.text().unwrap(),
        "Bearer stamped-token",
        "the middleware must apply the strategy's authorize() to every real request"
    );
}

/// A strategy is invoked independently per request — a second, differently
/// routed request must be stamped too, not just the first.
#[tokio::test]
async fn test_security_auth_middleware_stamps_repeated_requests_edge() {
    let (port, _jh) = spawn_once(|req| async move {
        let auth = req
            .headers()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("missing")
            .to_owned();
        Response::new(Full::new(Bytes::from(auth)))
    })
    .await;

    let (_d, l) = loader("[unrelated]\nx = 1");
    let strategy: Arc<dyn HttpEgressAuthStrategy> = Arc::new(StampingAuthStrategy);
    let egress = HttpTransportSvc::http_egress_from_config_with_auth(&l, strategy)
        .expect("auth-only egress must build");

    for path in ["/a", "/b"] {
        let resp = egress
            .send(HttpRequest::get(format!("http://127.0.0.1:{port}{path}")))
            .await
            .expect("reachable server must succeed");
        assert_eq!(resp.text().unwrap(), "Bearer stamped-token");
    }
}
