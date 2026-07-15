//! Cassette policy schema. Values live in `config/application.toml`.

use serde::Deserialize;

use crate::api::error::CassetteError;

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
#[derive(Debug, Clone, Deserialize)]
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

impl Default for CassetteConfig {
    fn default() -> Self {
        Self {
            mode: "replay".into(),
            cassette_dir: "tests/cassettes".into(),
            match_on: vec!["method".into(), "url".into(), "body_hash".into()],
            scrub_headers: vec![
                "authorization".into(),
                "x-api-key".into(),
                "cookie".into(),
                "set-cookie".into(),
                "proxy-authorization".into(),
            ],
            scrub_body_paths: vec![],
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for CassetteConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "cassette"
    }
}

/// Backend-owned opt-in contract (ADR-006): presence of the `[cassette]` section
/// activates HTTP record/replay; absence leaves it off. Additive alongside
/// [`swe_edge_configbuilder::ConfigSection`].
impl swe_edge_configbuilder::OptionalSection for CassetteConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "cassette"
    }

    fn metadata() -> swe_edge_configbuilder::FeatureMetadata {
        swe_edge_configbuilder::FeatureMetadata {
            description: "HTTP record/replay test fixtures",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}

impl CassetteConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, CassetteError> {
        toml::from_str(toml_text).map_err(|e| CassetteError::ParseFailed(e.to_string()))
    }

    /// A config that passes every request straight through — no recording,
    /// no replay, no cassette file I/O. Use in production stacks where
    /// record/replay infrastructure is not wanted.
    pub fn disabled() -> Self {
        Self {
            mode: "disabled".into(),
            cassette_dir: String::new(),
            match_on: vec![],
            scrub_headers: vec![],
            scrub_body_paths: vec![],
        }
    }

    /// Return the SWE default config (mode = "replay").
    ///
    /// Alias for `CassetteConfig::default()` — preferred in test code where
    /// the intent is to load the SWE baseline rather than construct
    /// an ad-hoc struct.
    pub fn swe_default() -> Result<Self, CassetteError> {
        Ok(Self::default())
    }
}
