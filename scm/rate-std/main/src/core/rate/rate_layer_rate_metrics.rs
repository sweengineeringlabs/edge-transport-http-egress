//! Public type — the rate-limiter middleware layer.

use std::sync::Arc;

use moka::future::Cache;

use edge_transport_http_egress_rate_api::RateConfig;

/// Rate-limiter middleware. Attach to a
/// `reqwest_middleware::ClientBuilder` via `.with(layer)`.
///
/// Named for the [`RateMetrics`](edge_transport_http_egress_rate_api::RateMetrics) contract it
/// implements: the layer exposes its configured rate limit through that trait.
pub(crate) struct RateLayerRateMetrics {
    pub(crate) config: Arc<RateConfig>,
    /// Per-host token buckets, keyed by authority
    /// (host:port). When `config.per_host = false`, a single
    /// bucket keyed by the sentinel key serves all requests.
    pub(crate) buckets: Cache<String, Arc<tokio::sync::Mutex<crate::core::token::TokenBucket>>>,
}
