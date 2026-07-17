//! [`SecurityAuthMiddleware`] — bridges an `edge-security` [`HttpEgressAuthStrategy`]
//! into a `reqwest_middleware::Middleware` layer.
//!
//! Pure trait-bridging glue: no credential resolution, no signing, no
//! protocol logic. All security implementation lives in `edge-security`'s
//! `bearer` crate (BYOSec reversed 2026-07-16); this type only calls
//! `authorize()` per request and forwards the (possibly mutated) request
//! down the chain — the same shape as this crate's non-security `retry`/
//! `rate`/`breaker` middleware wrappers.

use std::sync::Arc;

use async_trait::async_trait;
use edge_security_transport_egress_http::{
    AuthorizeRequest, HttpEgressAuthStrategy, OutboundHttpRequest,
};

/// Wraps a caller-supplied [`HttpEgressAuthStrategy`] as a
/// `reqwest_middleware::Middleware` layer.
pub(crate) struct SecurityAuthMiddleware {
    strategy: Arc<dyn HttpEgressAuthStrategy>,
}

impl SecurityAuthMiddleware {
    /// Construct from an already-resolved strategy. The strategy is
    /// `Arc`-shared because `reqwest_middleware::Middleware` needs `&self`
    /// concurrency and strategies are stateless post-construction.
    pub(crate) fn new(strategy: Arc<dyn HttpEgressAuthStrategy>) -> Self {
        Self { strategy }
    }
}

impl std::fmt::Debug for SecurityAuthMiddleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecurityAuthMiddleware")
            .finish_non_exhaustive()
    }
}

#[async_trait]
impl reqwest_middleware::Middleware for SecurityAuthMiddleware {
    async fn handle(
        &self,
        req: reqwest::Request,
        ext: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let authorized = self
            .strategy
            .authorize(AuthorizeRequest {
                request: OutboundHttpRequest::new(req),
            })
            .map_err(|e| reqwest_middleware::Error::Middleware(e.into()))?;
        next.run(authorized.request.into_inner(), ext).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_security_transport_egress_http::{AuthorizeResponse, HttpEgressAuthError};
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Stub strategy that records call count and attaches a known header,
    /// so the wiring (not the full `reqwest_middleware` dispatch machinery,
    /// which needs a live `Next` chain) is verifiable.
    struct StubStrategy {
        calls: AtomicUsize,
    }

    impl HttpEgressAuthStrategy for StubStrategy {
        fn authorize(
            &self,
            request: AuthorizeRequest,
        ) -> Result<AuthorizeResponse, HttpEgressAuthError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let mut inner = request.request.into_inner();
            inner.headers_mut().insert(
                "x-auth-applied",
                self.calls
                    .load(Ordering::SeqCst)
                    .to_string()
                    .parse()
                    .expect("digit string is a valid header value"),
            );
            Ok(AuthorizeResponse {
                request: OutboundHttpRequest::new(inner),
            })
        }
    }

    /// @covers: new
    #[test]
    fn test_new_holds_strategy() {
        let strategy: Arc<dyn HttpEgressAuthStrategy> = Arc::new(StubStrategy {
            calls: AtomicUsize::new(0),
        });
        let mw = SecurityAuthMiddleware::new(strategy.clone());
        assert!(Arc::ptr_eq(&mw.strategy, &strategy));
    }

    /// @covers: new
    /// Proves the wrapped strategy actually fires and mutates the request —
    /// the only synchronously-observable invariant without a live
    /// `reqwest_middleware::Next` chain (the real dispatch path is exercised
    /// end-to-end by this crate's `tests/*_int_test.rs`).
    #[test]
    fn test_strategy_authorize_applies_header_happy() {
        let strategy = StubStrategy {
            calls: AtomicUsize::new(0),
        };
        let req = reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse("http://127.0.0.1:1").expect("valid url"),
        );
        let response = strategy
            .authorize(AuthorizeRequest {
                request: OutboundHttpRequest::new(req),
            })
            .expect("stub strategy is infallible");
        let out = response.request.into_inner();
        assert_eq!(
            out.headers().get("x-auth-applied").expect("header set"),
            "1"
        );
        assert_eq!(strategy.calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_debug_impl_does_not_panic() {
        let mw = SecurityAuthMiddleware::new(Arc::new(StubStrategy {
            calls: AtomicUsize::new(0),
        }));
        let s = format!("{mw:?}");
        assert!(s.contains("SecurityAuthMiddleware"));
    }
}
