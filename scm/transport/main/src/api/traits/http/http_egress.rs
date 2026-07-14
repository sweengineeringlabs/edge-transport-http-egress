//! HTTP outbound trait — makes outbound HTTP requests.

use edge_application::SecurityContext;
use futures::future::BoxFuture;

use crate::api::types::http::http_egress_result::HttpEgressResult;
use crate::api::types::{HttpRequest, HttpResponse, HttpStreamResponse};

/// Makes outbound HTTP requests to external services.
pub trait HttpEgress: Send + Sync {
    /// Send an HTTP request and return the buffered response.
    fn send(&self, request: HttpRequest) -> BoxFuture<'_, HttpEgressResult<HttpResponse>>;

    /// Send an HTTP request, propagating the caller's security context to the outbound call.
    ///
    /// The default implementation delegates to [`send`] and ignores `_ctx`.
    /// Override to inject context-derived headers (e.g. `x-trace-id`, `Authorization`).
    ///
    /// [`send`]: HttpEgress::send
    fn send_with_context(
        &self,
        request: HttpRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpEgressResult<HttpResponse>> {
        self.send(request)
    }

    /// Send a request and return a lazy byte stream rather than a buffered body.
    ///
    /// Auth, rate-limit, and circuit-breaker middleware all apply to the initial
    /// connection. Retry middleware applies to the connection only — a
    /// partially-consumed stream cannot be transparently retried. If the stream
    /// drops mid-response, the caller must decide whether to reconnect.
    fn send_stream(
        &self,
        request: HttpRequest,
    ) -> BoxFuture<'_, HttpEgressResult<HttpStreamResponse>>;

    /// Probe reachability — returns `Ok(())` if the remote responds, error otherwise.
    fn health_check(&self) -> BoxFuture<'_, HttpEgressResult<()>>;

    /// Convenience shorthand for a GET request to `url`.
    fn get(&self, url: &str) -> BoxFuture<'_, HttpEgressResult<HttpResponse>> {
        let req = HttpRequest::get(url.to_string());
        self.send(req)
    }
}
