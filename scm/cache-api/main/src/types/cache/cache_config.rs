//! Cache policy schema — the struct layout, nothing else.
//!
//! Policy **values** live in TOML:
//! - crate-shipped baseline: `config/application.toml`
//! - workspace override: `edge/http/main/config/application.toml` under `[cache]`
//! - consumer override: whatever TOML the binary loads
//!
//! The impl blocks (`Default`, config-builder wiring, TOML parsing) live in
//! `core/cache/cache_config.rs` — api/ is a pure declaration layer.

use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
