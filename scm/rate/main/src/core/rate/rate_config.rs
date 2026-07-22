//! Impl blocks for [`RateConfig`] — defaults, config-builder wiring,
//! and TOML parsing.

use crate::api::{RateConfig, RateError};

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
/// alongside [`swe_edge_configbuilder::ConfigSection`].
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
