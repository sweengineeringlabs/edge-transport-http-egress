//! Request for [`crate::HttpEgress::config`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::HttpEgress::config`] — no parameters are needed;
/// the struct exists to satisfy the uniform request/response contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigRequest;
