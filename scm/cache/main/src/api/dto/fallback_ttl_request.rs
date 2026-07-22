//! Request for [`crate::api::HttpCache::default_ttl`].

use serde::{Deserialize, Serialize};

/// Input to [`crate::api::HttpCache::default_ttl`] — no parameters are needed;
/// the struct exists to satisfy the uniform request/response contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackTtlRequest;
