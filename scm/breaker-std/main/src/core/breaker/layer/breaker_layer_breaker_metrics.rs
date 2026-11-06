//! Public type — the circuit breaker middleware layer.

use std::sync::Arc;

use moka::future::Cache;

use edge_transport_http_egress_breaker_api::BreakerConfig;

/// Circuit breaker middleware. Attach to a
/// `reqwest_middleware::ClientBuilder` via `.with(layer)`.
pub(crate) struct BreakerLayerBreakerMetrics {
    pub(crate) config: Arc<BreakerConfig>,
    /// Per-host state, keyed by the URL's authority
    /// (host:port). `moka::future::Cache` gives us async-safe
    /// concurrent access with background expiration of
    /// long-idle entries.
    pub(crate) state:
        Cache<String, Arc<tokio::sync::Mutex<edge_transport_breaker_policy::BreakerNode>>>,
}
