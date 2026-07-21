//! HTTP outbound trait — makes outbound HTTP requests.

use async_trait::async_trait;

use crate::api::dto::{
    ConfigRequest, ConfigResponse, GetRequest, HealthCheckRequest, HttpRequest, HttpResponse,
    HttpStreamResponse,
};
use crate::api::error::HttpEgressError;
use crate::api::types::HttpSecurityContext;

/// Makes outbound HTTP requests to external services.
#[async_trait]
pub trait HttpEgress: Send + Sync {
    /// Send an HTTP request and return the buffered response.
    async fn send(&self, request: HttpRequest) -> Result<HttpResponse, HttpEgressError>;

    /// Send an HTTP request, propagating the caller's security context to the outbound call.
    ///
    /// The default implementation delegates to [`send`] and ignores `ctx`.
    /// Override to inject context-derived headers (e.g. `x-trace-id`, `Authorization`).
    ///
    /// [`send`]: HttpEgress::send
    async fn send_with_context(
        &self,
        request: HttpRequest,
        ctx: HttpSecurityContext,
    ) -> Result<HttpResponse, HttpEgressError> {
        let _ = ctx;
        self.send(request).await
    }

    /// Send a request and return a lazy byte stream rather than a buffered body.
    ///
    /// Auth, rate-limit, and circuit-breaker middleware all apply to the initial
    /// connection. Retry middleware applies to the connection only — a
    /// partially-consumed stream cannot be transparently retried. If the stream
    /// drops mid-response, the caller must decide whether to reconnect.
    async fn send_stream(
        &self,
        request: HttpRequest,
    ) -> Result<HttpStreamResponse, HttpEgressError>;

    /// Probe reachability — returns `Ok(())` if the remote responds, error otherwise.
    async fn health_check(&self, request: HealthCheckRequest) -> Result<(), HttpEgressError>;

    /// Convenience shorthand for a GET request to `request.url`.
    async fn get(&self, request: GetRequest) -> Result<HttpResponse, HttpEgressError> {
        let req = HttpRequest::get(request.url);
        self.send(req).await
    }

    /// Return this instance's [`HttpConfig`](crate::api::types::HttpConfig) —
    /// for introspection/observability (e.g. logging which endpoint an
    /// egress is configured against). Infallible — always returns `Ok`.
    fn config(&self, request: ConfigRequest) -> Result<ConfigResponse, HttpEgressError>;
}
