//! `impl` blocks for [`RetryConfig`] — defaults, TOML parsing, validation.
//! The type *declaration* lives in `api/`.

use crate::api::RetryConfig;
use crate::api::RetryError;

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_interval_ms: 200,
            max_interval_ms: 10000,
            multiplier: 2.0,
            retryable_statuses: vec![408, 425, 429, 500, 502, 503, 504],
            retryable_methods: vec!["GET".into(), "HEAD".into(), "PUT".into(), "DELETE".into()],
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for RetryConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "retry"
    }
}

/// Backend-owned opt-in contract (ADR-006): presence of the `[retry]` section
/// activates the retry middleware; absence leaves it off. Additive alongside
/// [`swe_edge_configbuilder::ConfigSection`].
impl swe_edge_configbuilder::OptionalSection for RetryConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "retry"
    }

    fn metadata() -> swe_edge_configbuilder::FeatureMetadata {
        swe_edge_configbuilder::FeatureMetadata {
            description: "Retry middleware with exponential backoff",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}

impl RetryConfig {
    /// Parse a retry config from TOML text.
    ///
    /// Returns [`RetryError::ParseFailed`] when the text isn't valid TOML,
    /// when a required key is missing, or when an unknown key is present
    /// (`deny_unknown_fields` is set — typos fail loudly rather than silently
    /// reverting to a default).
    pub fn from_config(toml_text: &str) -> Result<Self, RetryError> {
        toml::from_str(toml_text).map_err(|e| RetryError::ParseFailed(e.to_string()))
    }

    /// Validate the config fields.
    ///
    /// Returns `Err` with a human-readable description when any field is out
    /// of range (e.g. `multiplier` must be positive, `max_interval_ms` must be
    /// at least `initial_interval_ms`).
    pub fn validate(&self) -> Result<(), String> {
        if self.multiplier <= 0.0 {
            return Err(format!(
                "edge_transport_http_egress_retry: multiplier must be > 0.0, got {}",
                self.multiplier
            ));
        }
        if self.max_interval_ms < self.initial_interval_ms {
            return Err(format!(
                "edge_transport_http_egress_retry: max_interval_ms ({}) must be >= initial_interval_ms ({})",
                self.max_interval_ms, self.initial_interval_ms
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: from_config
    #[test]
    fn test_from_config_parses_valid_toml() {
        let cfg = RetryConfig::from_config(
            "max_retries = 5\ninitial_interval_ms = 100\nmax_interval_ms = 5000\nmultiplier = 1.5\nretryable_statuses = [503]\nretryable_methods = [\"GET\"]",
        )
        .expect("valid TOML must parse");
        assert_eq!(cfg.max_retries, 5);
        assert_eq!(cfg.initial_interval_ms, 100);
        assert!(cfg.retryable_statuses.contains(&503));
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_rejects_unknown_field() {
        let err = RetryConfig::from_config(
            "max_retries = 5\ninitial_interval_ms = 100\nmax_interval_ms = 5000\nmultiplier = 1.5\nretryable_statuses = [503]\nretryable_methods = [\"GET\"]\nbogus = 1",
        )
        .expect_err("unknown field must be rejected");
        assert!(
            matches!(err, RetryError::ParseFailed(_)),
            "unknown field must yield ParseFailed, got {err:?}"
        );
    }

    /// @covers: validate
    #[test]
    fn test_validate_default_is_ok() {
        assert!(RetryConfig::default().validate().is_ok());
        // Sibling negative case in the same test: an otherwise-default
        // config with multiplier=0 must fail, proving the is_ok() above
        // isn't a stub that always succeeds regardless of input.
        let invalid = RetryConfig {
            multiplier: 0.0,
            ..RetryConfig::default()
        };
        assert!(invalid.validate().is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_zero_multiplier() {
        let bad = RetryConfig {
            multiplier: 0.0,
            ..RetryConfig::default()
        };
        let msg = bad.validate().expect_err("multiplier=0 must fail");
        assert!(
            msg.contains("multiplier"),
            "error must mention multiplier: {msg}"
        );
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_max_interval_below_initial() {
        let bad = RetryConfig {
            initial_interval_ms: 1000,
            max_interval_ms: 100,
            ..RetryConfig::default()
        };
        assert!(bad.validate().is_err(), "max_interval < initial must fail");
    }
}
