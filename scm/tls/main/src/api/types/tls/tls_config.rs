//! `TlsConfig` — TLS identity policy schema.
//!
//! Values live in `config/application.toml`. No `Default` impl with literal
//! values — per the config-driven principle.

use serde::Deserialize;

use crate::api::error::TlsConfigError;

/// TLS client-identity schema. Tagged enum on `kind` so
/// `kind = "pkcs12"; path = "..."; password_env = "..."`
/// deserializes into the right variant.
///
/// Absence of the `[tls]` TOML section leaves the config at `None` — no client
/// cert is attached. Use `Pkcs12` for `.p12`/`.pfx` bundles (common with Java/OpenSSL
/// toolchains) or `Pem` for combined PEM files (common with Rust/Go toolchains).
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_tls::TlsConfig;
///
/// // Default: no client TLS.
/// let cfg = TlsConfig::default();
/// assert!(matches!(cfg, TlsConfig::None));
///
/// // PKCS#12 bundle (password from env var).
/// let cfg = TlsConfig::Pkcs12 {
///     path: "certs/client.p12".to_string(),
///     password_env: Some("CLIENT_CERT_PASSWORD".to_string()),
/// };
/// if let TlsConfig::Pkcs12 { path, password_env } = &cfg {
///     assert!(path.ends_with(".p12"));
///     assert!(password_env.is_some());
/// }
///
/// // Combined PEM file.
/// let cfg = TlsConfig::Pem { path: "certs/client.pem".to_string() };
/// assert!(matches!(cfg, TlsConfig::Pem { .. }));
///
/// // Parse from TOML.
/// let cfg = TlsConfig::from_config(r#"kind = "none""#).unwrap();
/// assert!(matches!(cfg, TlsConfig::None));
/// ```
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TlsConfig {
    /// Pass-through — no client cert attached. Baseline.
    #[default]
    None,

    /// PKCS#12 bundle (.p12 / .pfx) with cert + private key.
    Pkcs12 {
        /// Path to the .p12 / .pfx file.
        path: String,
        /// Env var holding the decryption password. Optional —
        /// omit for passwordless bundles.
        #[serde(default)]
        password_env: Option<String>,
    },

    /// Combined PEM file containing cert chain + private key.
    Pem {
        /// Path to the .pem file.
        path: String,
    },
}

impl swe_edge_configbuilder::ConfigSection for TlsConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "tls"
    }
}

/// Backend-owned opt-in contract (ADR-006): presence of the `[tls]` section
/// activates client TLS; absence leaves it off. Kept alongside `ConfigSection`
/// so existing direct construction keeps working during the migration.
impl swe_edge_configbuilder::OptionalSection for TlsConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "tls"
    }

    fn validate_enabled(&self) -> Result<(), swe_edge_configbuilder::ConfigError> {
        let path = match self {
            TlsConfig::None => return Ok(()),
            TlsConfig::Pkcs12 { path, .. } | TlsConfig::Pem { path } => path,
        };
        if path.trim().is_empty() {
            return Err(swe_edge_configbuilder::ConfigError::validation(
                <Self as swe_edge_configbuilder::OptionalSection>::section_name(),
                "tls identity `path` must be non-empty",
            ));
        }
        Ok(())
    }

    fn metadata() -> swe_edge_configbuilder::FeatureMetadata {
        swe_edge_configbuilder::FeatureMetadata {
            description: "client TLS identity (mutual TLS)",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}

impl crate::api::traits::Validator for TlsConfig {
    fn validate(&self) -> Result<(), String> {
        use swe_edge_configbuilder::OptionalSection;
        self.validate_enabled().map_err(|e| e.to_string())
    }
}

impl TlsConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, TlsConfigError> {
        toml::from_str(toml_text).map_err(|e| TlsConfigError::Config(e.to_string()))
    }
}
