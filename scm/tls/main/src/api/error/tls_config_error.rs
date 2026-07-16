//! `TlsConfigError` — errors raised while building or validating TLS config.

/// Errors raised while parsing, validating, or building TLS identity config.
#[derive(Debug, thiserror::Error)]
pub enum TlsConfigError {
    /// TLS config TOML didn't parse as the expected schema.
    #[error("edge_transport_http_egress_tls: config error — {0}")]
    Config(String),
    /// Config references an env var that isn't set.
    #[error("edge_transport_http_egress_tls: missing required environment variable {name}")]
    MissingEnvVar {
        /// Name of the missing env var.
        name: String,
    },
    /// Certificate/key file could not be read.
    #[error("edge_transport_http_egress_tls: failed to load certificate — {0}")]
    CertLoad(String),
    /// Certificate/key data could not be parsed.
    #[error("edge_transport_http_egress_tls: failed to parse certificate — {0}")]
    CertParse(String),
}
