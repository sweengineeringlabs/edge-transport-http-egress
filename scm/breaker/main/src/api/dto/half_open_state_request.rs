//! Request for [`crate::api::HostBreaker::is_half_open`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::HostBreaker::is_half_open`] — no parameters are
/// needed; the struct exists to satisfy the uniform request/response
/// contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HalfOpenStateRequest;
