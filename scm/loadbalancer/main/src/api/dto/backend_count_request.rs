//! Request for [`crate::api::PoolMetrics::backend_count`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::PoolMetrics::backend_count`] — no parameters are
/// needed; the struct exists to satisfy the uniform request/response contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendCountRequest;
