//! Error type for assembling a bare [`HttpEgress`](crate::HttpEgress) at startup.

use super::AssemblyFailure;

/// Error returned when assembling a bare [`HttpEgress`](crate::HttpEgress) /
/// [`HttpStream`](crate::HttpStream) fails at startup.
///
/// `transport` is a bare HTTP primitive (ADR-005/006): the only build-time failure it can surface is
/// the underlying reqwest client failing to construct. Composition-time failures —
/// retry/rate/breaker/cache/cassette (`edge-transport-http-egress-resilient`) and auth/oauth/tls
/// (`edge-security`) — belong to the composition root and its own error types, not here.
#[derive(Debug, thiserror::Error)]
pub enum HttpEgressBuildError {
    /// Reqwest client construction failed.
    #[error("reqwest: {0}")]
    Reqwest(AssemblyFailure),
}
