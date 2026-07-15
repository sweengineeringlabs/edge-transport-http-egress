//! Cache policy schema — the struct layout, nothing else.
//!
//! Policy **values** live in TOML:
//! - crate-shipped baseline: `config/application.toml`
//! - workspace override: `edge/http/main/config/application.toml` under `[cache]`
//! - consumer override: whatever TOML the binary loads
//!
//! No `Default` impl with literal numbers — per the
//! config-driven principle, policy is data in a file, not code
//! in a source tree.

use serde::Deserialize;

use crate::api::error::CacheError;

/// HTTP cache policy schema.
///
/// Implements RFC-7234 HTTP caching with a Moka in-memory LRU store. When
/// `respect_cache_control` is `true`, upstream `Cache-Control: no-store` and
/// `max-age` headers override `default_ttl_seconds`.
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_cache::CacheConfig;
///
/// // SWE baseline: 5-min TTL, 10k entries, honor Cache-Control.
/// let cfg = CacheConfig::default();
/// assert_eq!(cfg.default_ttl_seconds, 300);
/// assert_eq!(cfg.max_entries, 10_000);
/// assert!(cfg.respect_cache_control);
/// assert!(!cfg.cache_private);
///
/// // Custom policy from TOML.
/// let cfg = CacheConfig::from_config(
///     "default_ttl_seconds = 60\nmax_entries = 500\nrespect_cache_control = true\ncache_private = false"
/// ).unwrap();
/// assert_eq!(cfg.default_ttl_seconds, 60);
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CacheConfig {
    /// Fallback TTL when the upstream response lacks
    /// Cache-Control max-age.
    pub default_ttl_seconds: u64,
    /// Maximum entries in the in-memory store (LRU eviction).
    pub max_entries: u64,
    /// Honor upstream Cache-Control headers (no-store / max-age).
    pub respect_cache_control: bool,
    /// Cache responses marked `Cache-Control: private`.
    pub cache_private: bool,
}

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
