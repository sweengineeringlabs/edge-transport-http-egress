//! Public type — the circuit breaker middleware layer.

use std::sync::Arc;

use moka::future::Cache;

use crate::api::types::breaker::breaker_config::BreakerConfig;

/// Circuit breaker middleware. Attach to a
/// `reqwest_middleware::ClientBuilder` via `.with(layer)`.
pub struct BreakerLayerBreakerMetrics {
    pub(crate) config: Arc<BreakerConfig>,
    /// Per-host state, keyed by the URL's authority
    /// (host:port). `moka::future::Cache` gives us async-safe
    /// concurrent access with background expiration of
    /// long-idle entries.
    pub(crate) state: Cache<String, Arc<tokio::sync::Mutex<crate::core::host::DefaultHostBreaker>>>,
    /// Optional loadbalancer pool. When set (requires the `loadbalancer`
    /// feature), the breaker reports circuit-trip and recovery events back
    /// to the pool so that tripped backends are removed from rotation.
    #[cfg(feature = "loadbalancer")]
    pub(crate) pool: Option<Arc<swe_edge_loadbalancer::BackendPoolInstance>>,
}
