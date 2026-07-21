//! `CachedEntry` — the in-memory cached response entry.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// One cached response entry — the minimal shape needed to
/// reconstruct a response, plus RFC 7234 metadata.
#[derive(Clone, Debug)]
pub(crate) struct CachedEntry {
    /// HTTP status code captured at store time.
    pub(crate) status: u16,
    /// Response headers (snake-case keys).
    pub(crate) headers: BTreeMap<String, String>,
    /// Response body bytes.
    pub(crate) body: Arc<Vec<u8>>,
    /// Freshness deadline.
    pub(crate) expires_at: Instant,
    /// `ETag` header from the response, if any.
    pub(crate) etag: Option<String>,
    /// Vary header values captured at store time.
    pub(crate) vary_headers: Vec<(String, String)>,
    /// RFC 5861 `stale-while-revalidate` window.
    pub(crate) stale_while_revalidate: Option<Duration>,
}
