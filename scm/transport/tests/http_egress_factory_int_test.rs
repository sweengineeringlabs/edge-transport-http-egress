//! Integration tests for the SAF factory functions in `http_egress_factory`.
//!
//! Covers: `plain_http_egress`, `plain_http_egress_with_oauth`,
//! `default_http_stream_outbound`, and `validate_http_config`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{HttpConfig, HttpRequest, HttpStream, HttpTransportSvc};

// ─── plain_http_egress ──────────────────────────────────────────────────────

/// @covers: plain_http_egress
#[test]
fn test_plain_http_egress_builds_with_default_config() {
    let result = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(
        result.is_ok(),
        "plain_http_egress must build with default config: {:?}",
        result.err()
    );
}

/// @covers: plain_http_egress
#[test]
fn test_plain_http_egress_builds_with_custom_base_url() {
    let cfg = HttpConfig::with_base_url("https://custom.api.com");
    let result = HttpTransportSvc::plain_http_egress(cfg);
    assert!(
        result.is_ok(),
        "plain_http_egress must build with custom base URL: {:?}",
        result.err()
    );
}

// ─── default_http_stream_outbound ────────────────────────────────────────────

/// @covers: default_http_stream_outbound
#[test]
fn test_default_http_stream_outbound_builds_with_swe_defaults() {
    let result = HttpTransportSvc::default_http_stream_outbound();
    assert!(
        result.is_ok(),
        "default_http_stream_outbound must build: {:?}",
        result.err()
    );
}

/// @covers: default_http_stream_outbound
#[test]
fn test_default_http_stream_outbound_implements_stream_outbound_trait() {
    let outbound = HttpTransportSvc::default_http_stream_outbound().unwrap();
    fn _assert(_: &dyn HttpStream) {}
    _assert(outbound.as_ref());
}

// ─── validate_http_config ─────────────────────────────────────────────────────

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_returns_ok_for_valid_timeout() {
    let cfg = HttpConfig {
        timeout_secs: 30,
        connect_timeout_secs: 10,
        ..HttpConfig::default()
    };
    assert!(HttpTransportSvc::validate_http_config(&cfg).is_ok());
}

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_returns_err_for_zero_timeout() {
    let cfg = HttpConfig {
        timeout_secs: 0,
        ..HttpConfig::default()
    };
    let err = HttpTransportSvc::validate_http_config(&cfg).unwrap_err();
    assert!(
        err.contains("timeout_secs"),
        "error must name the offending field, got: {err:?}"
    );
}

// ─── plain_http_egress_with_oauth ────────────────────────────────────────────

#[cfg(feature = "oauth")]
mod oauth_factory {
    use std::convert::Infallible;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use bytes::Bytes;
    use edge_security_transport_egress_http_oauth::{
        AccessTokenRequest, AccessTokenResponse, OAuthError, OAuthTokenSource,
    };
    use http_body_util::Full;
    use hyper::body::Incoming;
    use hyper::server::conn::http1;
    use hyper::service::service_fn;
    use hyper::{Request, Response};
    use hyper_util::rt::TokioIo;

    use super::*;

    #[derive(Debug)]
    struct StaticTokenSource(String);

    #[async_trait::async_trait]
    impl OAuthTokenSource for StaticTokenSource {
        async fn get_access_token(
            &self,
            _request: AccessTokenRequest,
        ) -> Result<AccessTokenResponse, OAuthError> {
            Ok(AccessTokenResponse {
                token: self.0.clone(),
            })
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct FailingTokenSource;

    #[async_trait::async_trait]
    impl OAuthTokenSource for FailingTokenSource {
        async fn get_access_token(
            &self,
            _request: AccessTokenRequest,
        ) -> Result<AccessTokenResponse, OAuthError> {
            Err(OAuthError::CredentialsNotFound(
                "no credentials available".into(),
            ))
        }
    }

    async fn spawn_once_capturing_auth<F>(handler: F) -> (u16, tokio::task::JoinHandle<()>)
    where
        F: Fn(
                Request<Incoming>,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Response<Full<Bytes>>> + Send>>
            + Send
            + Clone
            + 'static,
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

    /// @covers: plain_http_egress_with_oauth
    #[test]
    fn test_plain_http_egress_with_oauth_builds_with_default_config() {
        let source = Arc::new(StaticTokenSource("test-token".into()));
        let result = HttpTransportSvc::plain_http_egress_with_oauth(HttpConfig::default(), source);
        assert!(
            result.is_ok(),
            "plain_http_egress_with_oauth must build with default config: {:?}",
            result.err()
        );
    }

    /// @covers: plain_http_egress_with_oauth
    /// Bearer token from `OAuthTokenSource` is injected into the `Authorization`
    /// header of every outbound request.
    #[tokio::test]
    async fn test_plain_http_egress_with_oauth_injects_bearer_token() {
        let captured: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
        let captured_clone = Arc::clone(&captured);

        let (port, _jh) = spawn_once_capturing_auth(move |req| {
            let auth = req
                .headers()
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned);
            *captured_clone.lock().unwrap() = auth;
            Box::pin(async {
                Response::builder()
                    .status(200)
                    .body(Full::new(Bytes::new()))
                    .unwrap()
            })
        })
        .await;

        let cfg = HttpConfig::with_base_url(format!("http://127.0.0.1:{port}"));
        let source = Arc::new(StaticTokenSource("secret-abc".into()));
        let egress = HttpTransportSvc::plain_http_egress_with_oauth(cfg, source).unwrap();
        let req = HttpRequest::get(format!("http://127.0.0.1:{port}/"));
        let _ = egress.send(req).await;

        let header = captured.lock().unwrap().clone().unwrap_or_default();
        assert_eq!(
            header, "Bearer secret-abc",
            "Authorization header must be 'Bearer secret-abc', got: {header:?}"
        );
    }

    /// @covers: plain_http_egress_with_oauth
    /// A server returning HTTP 401 maps to `HttpEgressError::Unauthorized`.
    #[tokio::test]
    async fn test_plain_http_egress_with_oauth_returns_unauthorized_on_401() {
        let (port, _jh) = spawn_once_capturing_auth(|_req| {
            Box::pin(async {
                Response::builder()
                    .status(401)
                    .body(Full::new(Bytes::new()))
                    .unwrap()
            })
        })
        .await;

        let cfg = HttpConfig::with_base_url(format!("http://127.0.0.1:{port}"));
        let source = Arc::new(StaticTokenSource("expired-token".into()));
        let egress = HttpTransportSvc::plain_http_egress_with_oauth(cfg, source).unwrap();
        let req = HttpRequest::get(format!("http://127.0.0.1:{port}/"));
        let err = egress.send(req).await.unwrap_err();
        assert!(
            matches!(
                err,
                edge_transport_http_egress_transport::HttpEgressError::Unauthorized(_)
            ),
            "HTTP 401 must map to HttpEgressError::Unauthorized, got: {err:?}"
        );
    }
}
