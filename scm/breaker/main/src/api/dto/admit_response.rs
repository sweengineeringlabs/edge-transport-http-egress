//! Response for [`crate::api::CircuitBreakerNode::admit`].

use serde::{Deserialize, Serialize};

use crate::api::Admission;

/// Output of [`crate::api::CircuitBreakerNode::admit`] — the admission
/// decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmitResponse {
    /// The admission decision.
    pub admission: Admission,
}
