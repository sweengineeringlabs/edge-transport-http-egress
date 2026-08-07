//! Retry policy schema — the struct layout, nothing else.
//!
//! Policy **values** live in TOML:
//! - crate-shipped baseline: `config/application.toml`
//! - workspace override: `edge/http/main/config/application.toml` under `[retry]`
//! - consumer override: whatever TOML the binary loads and passes
//!   to `RetryConfig::from_config`.
//!
//! This file is a pure declaration — the `Default` impl, TOML parsing,
//! and validation all live in `core/`.

use serde::{Deserialize, Serialize};

/// Retry policy schema. Deserialized from TOML via
/// [`RetryConfig::from_config`](crate::api::RetryConfig). Consumers compose
/// this into a middleware layer via `HttpRetrySvc.decorate(DecorateRequest { config })`.
///
/// Only idempotent methods (GET, HEAD, PUT, DELETE) are retried by default.
/// Only the listed `retryable_statuses` trigger a retry; 4xx errors (except
/// 408/425/429) are never retried.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetryConfig {
    /// Maximum attempts per request (1 = no retry).
    pub max_retries: u32,

    /// Delay before the first retry, in milliseconds.
    pub initial_interval_ms: u64,

    /// Upper bound on any single retry interval, in milliseconds.
    pub max_interval_ms: u64,

    /// Exponential backoff base (e.g. 2.0 → 200ms, 400ms, 800ms).
    pub multiplier: f64,

    /// Jitter as a fraction of the computed backoff (`0.0` = none, `0.1` = up to 10% random
    /// delta). Defaults to `0.0` (deterministic backoff, this crate's historical behavior) so
    /// existing TOML configs without this field keep working unchanged.
    #[serde(default)]
    pub jitter_factor: f64,

    /// HTTP status codes that trigger a retry.
    pub retryable_statuses: Vec<u16>,

    /// HTTP methods that can safely be retried.
    pub retryable_methods: Vec<String>,
}
