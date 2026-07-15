//! `CacheError` — domain error for the cache middleware.

/// Errors raised by the cache middleware.
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    /// Config TOML didn't parse as the expected schema.
    #[error("edge_transport_http_egress_cache: config parse failed — {0}")]
    ParseFailed(String),
}
