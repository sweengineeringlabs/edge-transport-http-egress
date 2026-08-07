//! Impl blocks for [`BreakerConfig`] — defaults, config-builder wiring,
//! and TOML parsing.

use crate::api::{BreakerConfig, BreakerError};

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

impl From<BreakerConfig> for edge_transport_breaker_policy::BreakerConfig {
    fn from(cfg: BreakerConfig) -> Self {
        Self {
            failure_threshold: cfg.failure_threshold,
            cool_down_seconds: cfg.half_open_after_seconds,
            half_open_probe_count: cfg.reset_after_successes,
        }
    }
}
