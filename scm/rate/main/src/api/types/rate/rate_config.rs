//! Client-side rate-limiter policy schema. Values live in
//! `config/application.toml`.

use serde::{Deserialize, Serialize};

/// Rate-limiter (token-bucket) policy schema.
///
/// Applies a token-bucket rate limiter client-side before sending requests.
/// When `per_host` is `true`, each target host gets its own bucket; `false`
/// applies a single global bucket across all hosts.
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_rate::RateConfig;
///
/// // SWE baseline: 10 req/s, 20-request burst, per-host buckets.
/// let cfg = RateConfig::default();
/// assert_eq!(cfg.tokens_per_second, 10);
/// assert_eq!(cfg.burst_capacity, 20);
/// assert!(cfg.per_host);
///
/// // Custom TOML.
/// let cfg = RateConfig::from_config(
///     "tokens_per_second = 5\nburst_capacity = 10\nper_host = false"
/// ).unwrap();
/// assert_eq!(cfg.tokens_per_second, 5);
/// assert!(!cfg.per_host);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RateConfig {
    /// Sustained refill rate, tokens per second.
    pub tokens_per_second: u32,
    /// Bucket capacity (burst tolerance).
    pub burst_capacity: u32,
    /// Per-host bucketing (false = single global bucket).
    pub per_host: bool,
}
