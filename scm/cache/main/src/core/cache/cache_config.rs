//! Impl blocks for [`CacheConfig`] — defaults, config-builder wiring, and
//! TOML parsing. api/ declares the schema; the behaviour lives here.

use crate::api::{CacheConfig, CacheError};

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            default_ttl_seconds: 300,
            max_entries: 10000,
            respect_cache_control: true,
            cache_private: false,
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for CacheConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "cache"
    }
}

/// Backend-owned opt-in contract (ADR-006): presence of the `[cache]` section
/// activates the HTTP response cache; absence leaves it off. Additive alongside
/// [`swe_edge_configbuilder::ConfigSection`].
impl swe_edge_configbuilder::OptionalSection for CacheConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "cache"
    }

    fn metadata() -> swe_edge_configbuilder::FeatureMetadata {
        swe_edge_configbuilder::FeatureMetadata {
            description: "HTTP response cache",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}

impl CacheConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, CacheError> {
        toml::from_str(toml_text).map_err(|e| CacheError::ParseFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: default
    #[test]
    fn test_default_uses_swe_baseline_values() {
        let cfg = CacheConfig::default();
        assert_eq!(cfg.default_ttl_seconds, 300);
        assert_eq!(cfg.max_entries, 10000);
        assert!(cfg.respect_cache_control);
        assert!(!cfg.cache_private);
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_parses_valid_toml() {
        let cfg = CacheConfig::from_config(
            "default_ttl_seconds = 60\nmax_entries = 500\nrespect_cache_control = true\ncache_private = false",
        )
        .expect("valid toml must parse");
        assert_eq!(cfg.default_ttl_seconds, 60);
        assert_eq!(cfg.max_entries, 500);
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_rejects_unknown_field() {
        let err = CacheConfig::from_config(
            "default_ttl_seconds = 60\nmax_entries = 500\nrespect_cache_control = true\ncache_private = false\nbogus = 1",
        )
        .expect_err("unknown field must be rejected");
        assert!(matches!(err, CacheError::ParseFailed(_)));
    }
}
