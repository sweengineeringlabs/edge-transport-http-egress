//! Error type for assembling an [`HttpEgress`](crate::HttpEgress) at startup.

/// Error returned when assembling an [`HttpEgress`](crate::HttpEgress) fails at startup.
///
/// There is no `Auth` variant: since BYOSec was reversed (2026-07-16), the
/// auth strategy is caller-constructed and passed in already-built (see
/// [`HttpTransportSvc::http_egress_from_config_with_auth`](crate::api::types::HttpTransportSvc::http_egress_from_config_with_auth)),
/// so this crate never constructs one itself and has no build-time auth
/// failure to report.
#[derive(Debug, thiserror::Error)]
pub enum HttpEgressBuildError {
    /// Retry middleware assembly failed.
    #[cfg(feature = "retry")]
    #[error("retry: {0}")]
    Retry(#[from] edge_transport_http_egress_retry::RetryError),
    /// Rate-limiting middleware assembly failed.
    #[cfg(feature = "rate")]
    #[error("rate: {0}")]
    Rate(#[from] edge_transport_http_egress_rate::RateError),
    /// Circuit-breaker middleware assembly failed.
    #[cfg(feature = "breaker")]
    #[error("breaker: {0}")]
    Breaker(#[from] edge_transport_http_egress_breaker::BreakerError),
    /// Cache middleware assembly failed.
    #[cfg(feature = "cache")]
    #[error("cache: {0}")]
    Cache(#[from] edge_transport_http_egress_cache::CacheError),
    /// Cassette middleware assembly failed.
    #[cfg(feature = "cassette")]
    #[error("cassette: {0}")]
    Cassette(#[from] edge_transport_http_egress_cassette::CassetteError),
    /// TLS middleware assembly failed.
    #[cfg(feature = "tls")]
    #[error("tls: {0}")]
    Tls(#[from] edge_transport_http_egress_tls::TlsConfigError),
    /// OAuth builder assembly failed.
    #[cfg(feature = "oauth")]
    #[error("oauth: {0}")]
    OAuth(#[from] edge_transport_http_egress_oauth::OAuthError),
    /// Reqwest client construction failed.
    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Loading or validating an optional `[section]` from config failed.
    #[error("config: {0}")]
    Config(#[from] swe_edge_configbuilder::ConfigError),
}
