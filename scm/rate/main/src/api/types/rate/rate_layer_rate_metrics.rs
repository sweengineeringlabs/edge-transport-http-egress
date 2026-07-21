//! Public type — the rate-limiter middleware layer.

use std::sync::Arc;

use moka::future::Cache;

use crate::api::types::rate::rate_config::RateConfig;

/// Rate-limiter middleware. Attach to a
/// `reqwest_middleware::ClientBuilder` via `.with(layer)`.
///
/// Named for the [`RateMetrics`](crate::api::RateMetrics) contract it
/// implements: the layer exposes its configured rate limit through that trait.
pub struct RateLayerRateMetrics {
    pub(crate) config: Arc<RateConfig>,
    /// Per-host token buckets, keyed by authority
    /// (host:port). When `config.per_host = false`, a single
    /// bucket keyed by the sentinel key serves all requests.
    pub(crate) buckets: Cache<String, Arc<tokio::sync::Mutex<crate::core::token::TokenBucket>>>,
}
