//! Public type — the HTTP cache middleware layer.

use std::sync::Arc;

use moka::future::Cache;

use super::cache_config::CacheConfig;

/// HTTP cache middleware. Attach to a
/// `reqwest_middleware::ClientBuilder` via `.with(layer)`.
///
/// Simple TTL-based cache — see `core::cache_layer` module
/// docs for the covered + uncovered RFC 7234 semantics.
pub struct MiddlewareHttpCache {
    pub(crate) config: Arc<CacheConfig>,
    /// Primary store: `(method, url)` → Vec of CachedEntry variants
    /// (one variant per observed `Vary` combination). Wrapped in
    /// `Arc` so the moka value type stays cheap to clone on
    /// read-side copies.
    pub(crate) store: Cache<String, Arc<Vec<crate::core::cached::entry::CachedEntry>>>,
    /// Client used for `stale-while-revalidate` background
    /// refreshes. The spawned refresh task cannot re-enter the
    /// middleware chain (`reqwest_middleware::Next<'a>` is
    /// non-`'static`), so SWR refreshes go out over this bare
    /// client — bypassing any other middleware in the chain.
    /// This is a documented limitation.
    pub(crate) swr_client: Arc<reqwest::Client>,
}
