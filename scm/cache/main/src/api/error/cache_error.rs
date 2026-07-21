//! `CacheError` — domain error for the cache middleware.

/// Errors raised by the cache middleware.
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    /// Config TOML didn't parse as the expected schema.
    #[error("edge_transport_http_egress_cache: config parse failed — {0}")]
    ParseFailed(String),

    /// A [`CacheConfig`](crate::api::CacheConfig) field failed structural
    /// validation (e.g. a zero `max_entries`).
    #[error("edge_transport_http_egress_cache: invalid config — {0}")]
    InvalidConfig(String),
}
