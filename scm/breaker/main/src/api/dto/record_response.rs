//! Response for [`crate::api::CircuitBreakerNode::record`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::api::CircuitBreakerNode::record`] — recording an
/// outcome has no observable output; the struct exists to satisfy the
/// uniform request/response contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordResponse;
