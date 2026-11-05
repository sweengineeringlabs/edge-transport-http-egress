//! The result of inspecting a response for cacheability.

use std::time::Duration;

/// The result of inspecting a response for cacheability: a fresh
/// TTL plus an optional SWR window. `None` means "do not cache."
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TtlDecision {
    pub(crate) ttl: Duration,
    pub(crate) swr: Option<Duration>,
}
