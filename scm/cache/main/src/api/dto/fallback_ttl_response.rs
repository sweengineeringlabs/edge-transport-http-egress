//! Response for [`crate::api::HttpCache::default_ttl`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::api::HttpCache::default_ttl`] — the fallback TTL, in
/// seconds, that the cache layer applies when an upstream response carries no
/// `Cache-Control: max-age`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackTtlResponse {
    /// The configured fallback TTL, in seconds.
    pub seconds: u64,
}
