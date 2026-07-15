//! Client-side rate-limiter policy schema. Values live in
//! `config/application.toml`.

use serde::Deserialize;

use crate::api::error::RateError;

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
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RateConfig {
    /// Sustained refill rate, tokens per second.
    pub tokens_per_second: u32,
    /// Bucket capacity (burst tolerance).
    pub burst_capacity: u32,
    /// Per-host bucketing (false = single global bucket).
    pub per_host: bool,
}

impl Default for RateConfig {
    fn default() -> Self {
        Self {
            tokens_per_second: 10,
            burst_capacity: 20,
            per_host: true,
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for RateConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "rate"
    }
}

/// Backend-owned opt-in contract (ADR-006): presence of the `[rate]` section
/// activates client-side rate limiting; absence leaves it off. Additive
/// alongside `ConfigSection`.
impl swe_edge_configbuilder::OptionalSection for RateConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "rate"
    }

    fn metadata() -> swe_edge_configbuilder::FeatureMetadata {
        swe_edge_configbuilder::FeatureMetadata {
            description: "client-side request rate limiting",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}

impl RateConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, RateError> {
        toml::from_str(toml_text).map_err(|e| RateError::ParseFailed(e.to_string()))
    }
}
