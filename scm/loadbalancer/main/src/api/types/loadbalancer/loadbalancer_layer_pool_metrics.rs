//! `LoadbalancerLayerPoolMetrics` — the public middleware layer type.

use std::sync::Arc;

use swe_edge_loadbalancer::BackendPoolInstance;

/// Load-balancer middleware. Attach to a `reqwest_middleware::ClientBuilder`
/// via `.with(layer)`.
///
/// Constructed via [`crate::LoadbalancerSvcProcessor::build_layer`].
///
/// On each request, selects a healthy backend from the pool and rewrites the
/// request URL (scheme + host + port) to point to that backend while
/// preserving the original path, query, and fragment. Implements
/// [`PoolMetrics`](crate::api::PoolMetrics) so callers can inspect the pool.
pub struct LoadbalancerLayerPoolMetrics {
    pub(crate) pool: Arc<BackendPoolInstance>,
}
