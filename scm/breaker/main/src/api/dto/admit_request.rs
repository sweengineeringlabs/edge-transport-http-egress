//! Request for [`crate::api::CircuitBreakerNode::admit`].

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::api::BreakerConfig;

/// Input to [`crate::api::CircuitBreakerNode::admit`] — the active
/// breaker policy to evaluate the current state against.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdmitRequest {
    /// The active breaker policy.
    pub config: Arc<BreakerConfig>,
}
