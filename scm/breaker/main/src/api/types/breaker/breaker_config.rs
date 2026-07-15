//! Circuit-breaker policy schema. Values live in
//! `config/application.toml`.

use serde::Deserialize;

use crate::api::error::BreakerError;

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
#[derive(Debug, Clone, Deserialize)]
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

impl Default for BreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            half_open_after_seconds: 30,
            reset_after_successes: 3,
            failure_statuses: vec![500, 502, 503, 504],
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for BreakerConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "breaker"
    }
}

/// Backend-owned opt-in contract (ADR-006): presence of the `[breaker]` section
/// activates the circuit breaker; absence leaves it off. Additive alongside
/// [`swe_edge_configbuilder::ConfigSection`].
impl swe_edge_configbuilder::OptionalSection for BreakerConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "breaker"
    }

    fn metadata() -> swe_edge_configbuilder::FeatureMetadata {
        swe_edge_configbuilder::FeatureMetadata {
            description: "circuit breaker for failing upstreams",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}

impl BreakerConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, BreakerError> {
        toml::from_str(toml_text).map_err(|e| BreakerError::ParseFailed(e.to_string()))
    }
}
