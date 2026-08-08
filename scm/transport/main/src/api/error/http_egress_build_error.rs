//! Error type for assembling an [`HttpEgress`](crate::HttpEgress) at startup.

use super::AssemblyFailure;

/// Error returned when assembling an [`HttpEgress`](crate::HttpEgress) fails at startup.
///
/// There is no `Auth` variant: since BYOSec was reversed (2026-07-16), the
/// auth strategy is caller-constructed and passed in already-built (see
/// [`HttpTransportSvc::http_egress_from_config_with_auth`](crate::api::types::HttpTransportSvc::http_egress_from_config_with_auth)),
/// so this crate never constructs one itself and has no build-time auth
/// failure to report.
///
/// Each variant wraps its underlying sibling-crate error in the local
/// [`AssemblyFailure`] newtype (SEA `no_foreign_type`) — the `From<ForeignError>`
/// conversions used by the `?` operator at every call site live in `core/`.
#[derive(Debug, thiserror::Error)]
pub enum HttpEgressBuildError {
    /// The retry/rate/breaker/cache/cassette chain (`edge-transport-http-egress-resilient`)
    /// failed to assemble.
    #[error("resilient: {0}")]
    Resilient(AssemblyFailure),
    /// TLS middleware assembly failed.
    #[cfg(feature = "tls")]
    #[error("tls: {0}")]
    Tls(AssemblyFailure),
    /// OAuth builder assembly failed.
    #[cfg(feature = "oauth")]
    #[error("oauth: {0}")]
    OAuth(AssemblyFailure),
    /// Reqwest client construction failed.
    #[error("reqwest: {0}")]
    Reqwest(AssemblyFailure),
    /// Loading or validating an optional `[section]` from config failed.
    #[error("config: {0}")]
    Config(AssemblyFailure),
}
