//! Response for [`crate::api::PoolMetrics::backend_count`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::api::PoolMetrics::backend_count`] — the number of
/// backends currently in the pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendCountResponse {
    /// The backend count.
    pub value: usize,
}
