//! `HttpCache` — the cache middleware's state-inspection contract.

use crate::api::{CacheError, FallbackTtlRequest, FallbackTtlResponse};

/// The cache crate's domain trait. The middleware layer produced by this crate
/// implements it, exposing the resolved policy for inspection.
pub trait HttpCache: Send + Sync {
    /// Return the fallback TTL (in seconds) configured for this cache layer —
    /// the value applied when an upstream response carries no
    /// `Cache-Control: max-age`.
    fn default_ttl(&self, request: FallbackTtlRequest) -> Result<FallbackTtlResponse, CacheError>;
}
