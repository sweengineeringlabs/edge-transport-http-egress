//! HTTP outbound trait — makes outbound HTTP requests.

use super::HttpEgressFuture;
use crate::api::dto::{
    ConfigRequest, ConfigResponse, GetRequest, HealthCheckRequest, HttpRequest, HttpResponse,
    HttpStreamResponse,
};
use crate::api::error::HttpEgressError;
use crate::api::types::HttpSecurityContext;

/// Makes outbound HTTP requests to external services.
pub trait HttpEgress: Send + Sync {
    /// Send an HTTP request with an unauthenticated context and return the buffered response.
    ///
    /// This convenience method delegates to [`send_with_context`]. Implementors only need to
    /// provide the context-aware operation.
    ///
    /// [`send_with_context`]: HttpEgress::send_with_context
    fn send<'life0, 'async_trait>(
        &'life0 self,
        request: HttpRequest,
    ) -> HttpEgressFuture<'async_trait, HttpResponse>
    where
        'life0: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            self.send_with_context(request, HttpSecurityContext::unauthenticated())
                .await
        })
    }

    /// Send an HTTP request with the caller's security context.
    ///
    /// Implementations that own context-dependent behavior must consume this context. The bare
    /// reqwest transport does not interpret security policy; composition roots apply auth and
    /// context-derived headers in their middleware stack.
    fn send_with_context<'life0, 'async_trait>(
        &'life0 self,
        request: HttpRequest,
        ctx: HttpSecurityContext,
    ) -> HttpEgressFuture<'async_trait, HttpResponse>
    where
        'life0: 'async_trait,
        Self: Sync + 'async_trait;

    /// Send a request and return a lazy byte stream rather than a buffered body.
    ///
    /// Auth, rate-limit, and circuit-breaker middleware all apply to the initial
    /// connection. Retry middleware applies to the connection only — a
    /// partially-consumed stream cannot be transparently retried. If the stream
    /// drops mid-response, the caller must decide whether to reconnect.
    fn send_stream<'life0, 'async_trait>(
        &'life0 self,
        request: HttpRequest,
    ) -> HttpEgressFuture<'async_trait, HttpStreamResponse>
    where
        'life0: 'async_trait,
        Self: Sync + 'async_trait;

    /// Probe reachability — returns `Ok(())` if the remote responds, error otherwise.
    fn health_check<'life0, 'async_trait>(
        &'life0 self,
        request: HealthCheckRequest,
    ) -> HttpEgressFuture<'async_trait, ()>
    where
        'life0: 'async_trait,
        Self: Sync + 'async_trait;

    /// Convenience shorthand for a GET request to `request.url`.
    fn get<'life0, 'async_trait>(
        &'life0 self,
        request: GetRequest,
    ) -> HttpEgressFuture<'async_trait, HttpResponse>
    where
        'life0: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let req = HttpRequest::get(request.url);
            self.send(req).await
        })
    }

    /// Return this instance's [`HttpConfig`](crate::api::types::HttpConfig) —
    /// for introspection/observability (e.g. logging which endpoint an
    /// egress is configured against). Infallible — always returns `Ok`.
    fn config(&self, request: ConfigRequest) -> Result<ConfigResponse, HttpEgressError>;
}
