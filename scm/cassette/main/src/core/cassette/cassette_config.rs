//! Impl blocks for [`CassetteConfig`] — defaults, config-builder wiring, and
//! TOML parsing. api/ declares the schema; the behaviour lives here.

use crate::api::{CassetteConfig, CassetteError};

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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: default
    #[test]
    fn test_default_uses_swe_baseline_values() {
        let cfg = CassetteConfig::default();
        assert_eq!(cfg.mode, "replay");
        assert_eq!(cfg.cassette_dir, "tests/cassettes");
        assert!(cfg.scrub_headers.contains(&"authorization".to_string()));
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_parses_valid_toml() {
        let cfg = CassetteConfig::from_config(
            r#"mode = "record"
cassette_dir = "tests/fixtures"
match_on = ["method", "url"]
scrub_headers = ["x-api-key"]
scrub_body_paths = []"#,
        )
        .expect("valid toml must parse");
        assert_eq!(cfg.mode, "record");
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_rejects_unknown_field() {
        let err = CassetteConfig::from_config(
            r#"mode = "record"
cassette_dir = "tests/fixtures"
match_on = []
scrub_headers = []
scrub_body_paths = []
bogus = 1"#,
        )
        .expect_err("unknown field must be rejected");
        assert!(matches!(err, CassetteError::ParseFailed(_)));
    }

    /// @covers: disabled
    #[test]
    fn test_disabled_produces_pass_through_config() {
        let cfg = CassetteConfig::disabled();
        assert_eq!(cfg.mode, "disabled");
        assert!(cfg.match_on.is_empty());
    }

    /// @covers: swe_default
    #[test]
    fn test_swe_default_matches_default_impl() {
        let cfg = CassetteConfig::swe_default().expect("must succeed");
        assert_eq!(cfg.mode, CassetteConfig::default().mode);
    }
}
