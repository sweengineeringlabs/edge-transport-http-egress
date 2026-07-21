//! Request for [`crate::api::CircuitBreakerNode::record`].

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::api::{BreakerConfig, Outcome};

/// Input to [`crate::api::CircuitBreakerNode::record`] — the active
/// breaker policy and the outcome to apply.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordRequest {
    /// The active breaker policy.
    pub config: Arc<BreakerConfig>,
    /// The outcome to apply.
    pub outcome: Outcome,
}
