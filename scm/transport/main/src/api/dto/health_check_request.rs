//! Request for [`crate::api::HttpEgress::health_check`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::HttpEgress::health_check`] — no parameters are
/// needed; the struct exists to satisfy the uniform request/response
/// contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthCheckRequest;
