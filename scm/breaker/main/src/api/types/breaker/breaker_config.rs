//! Circuit-breaker policy schema. Values live in
//! `config/application.toml`.

use serde::{Deserialize, Serialize};

/// Circuit-breaker policy schema.
///
/// The breaker trips open when `failure_threshold` consecutive responses match
/// `failure_statuses` (default: 500/502/503/504). After `half_open_after_seconds`
/// it admits one probe; `reset_after_successes` consecutive successes close it.
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_breaker::BreakerConfig;
///
/// // SWE baseline: 5 failures, 30s cool-down, 3 probes to reset.
/// let cfg = BreakerConfig::default();
/// assert_eq!(cfg.failure_threshold, 5);
/// assert_eq!(cfg.failure_statuses, vec![500, 502, 503, 504]);
///
/// // Custom TOML.
/// let cfg = BreakerConfig::from_config(
///     "failure_threshold = 3\nhalf_open_after_seconds = 10\nreset_after_successes = 2\nfailure_statuses = [503, 504]"
/// ).unwrap();
/// assert_eq!(cfg.failure_threshold, 3);
/// assert_eq!(cfg.failure_statuses, vec![503, 504]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BreakerConfig {
    /// Consecutive failures that trip the breaker open.
    pub failure_threshold: u32,
    /// Seconds to wait after opening before probing.
    pub half_open_after_seconds: u64,
    /// Consecutive probe successes required to close.
    pub reset_after_successes: u32,
    /// Response statuses counted as failures.
    pub failure_statuses: Vec<u16>,
}
