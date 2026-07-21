//! Cassette policy schema. Values live in `config/application.toml`.
//!
//! The impl blocks (`Default`, config-builder wiring, TOML parsing) live in
//! `core/cassette/cassette_config.rs` — api/ is a pure declaration layer.

use serde::{Deserialize, Serialize};

/// Cassette record/replay policy schema.
///
/// Controls whether requests are recorded to disk (`"record"`), replayed from
/// disk (`"replay"`), automatically switched based on cassette presence (`"auto"`),
/// or bypassed entirely (`"disabled"`). Use `disabled()` in production factory
/// functions to get a zero-cost pass-through.
///
/// Credentials in headers and body paths listed in `scrub_headers` /
/// `scrub_body_paths` are zeroed before the cassette is written — cassettes
/// committed to VCS never contain real secrets.
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_cassette::CassetteConfig;
///
/// // Production: disabled pass-through (no cassette I/O).
/// let cfg = CassetteConfig::disabled();
/// assert_eq!(cfg.mode, "disabled");
/// assert!(cfg.match_on.is_empty());
///
/// // Test default: replay from tests/cassettes/.
/// let cfg = CassetteConfig::default();
/// assert_eq!(cfg.mode, "replay");
/// assert_eq!(cfg.cassette_dir, "tests/cassettes");
/// assert!(cfg.scrub_headers.contains(&"authorization".to_string()));
///
/// // Custom from TOML.
/// let cfg = CassetteConfig::from_config(
///     r#"mode = "record"
/// cassette_dir = "tests/fixtures"
/// match_on = ["method", "url"]
/// scrub_headers = ["x-api-key"]
/// scrub_body_paths = []"#
/// ).unwrap();
/// assert_eq!(cfg.mode, "record");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CassetteConfig {
    /// Operating mode: `"replay"` | `"record"` | `"auto"` | `"disabled"`.
    ///
    /// `"disabled"` passes every request straight through without touching
    /// any cassette file. Use this in production factory functions where
    /// record/replay is not wanted.
    pub mode: String,
    /// Cassette directory (relative to the test binary's manifest).
    pub cassette_dir: String,
    /// Request components included in the match key.
    pub match_on: Vec<String>,
    /// Headers to strip before writing the cassette.
    pub scrub_headers: Vec<String>,
    /// JSON paths inside the request body to zero out before
    /// hashing (e.g. `"request_id"`, `"metadata.trace_id"`).
    pub scrub_body_paths: Vec<String>,
}
