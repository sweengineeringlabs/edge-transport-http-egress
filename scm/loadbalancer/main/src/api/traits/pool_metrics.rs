//! `PoolMetrics` — backend-pool inspection contract for the loadbalancer layer.

use crate::api::{BackendCountRequest, BackendCountResponse, LoadbalancerMiddlewareError};

/// Read-only inspection of the backend pool backing a loadbalancer layer.
pub trait PoolMetrics: Send + Sync {
    /// Return the number of backends currently in the pool.
    fn backend_count(
        &self,
        request: BackendCountRequest,
    ) -> Result<BackendCountResponse, LoadbalancerMiddlewareError>;
}
