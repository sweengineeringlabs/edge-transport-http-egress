//! `TlsError` — error type for the tls middleware.

/// Errors raised by edge_transport_http_egress_tls.
#[derive(Debug, thiserror::Error)]
pub enum TlsError {
    /// Config TOML didn't parse as the expected schema.
    #[error("edge_transport_http_egress_tls: config parse failed — {0}")]
    ParseFailed(String),

    /// Config references an env var that isn't set.
    #[error("edge_transport_http_egress_tls: required env var {name} is not set")]
    MissingEnvVar {
        /// The missing env-var name.
        name: String,
    },

    /// Certificate / key file couldn't be read from disk. Wraps
    /// the underlying IO error message for operators to
    /// diagnose (wrong path, permissions, etc.).
    #[error("edge_transport_http_egress_tls: could not read file {path:?} — {reason}")]
    FileReadFailed {
        /// Path the config referenced.
        path: String,
        /// Underlying I/O error.
        reason: String,
    },

    /// Certificate / key data is present but not parseable as
    /// the declared format. E.g. PKCS12 with wrong password,
    /// PEM with malformed blocks.
    #[error("edge_transport_http_egress_tls: invalid {format} data — {reason}")]
    InvalidCertificate {
        /// Format we tried to parse as (pkcs12, pem).
        format: &'static str,
        /// Underlying parse error.
        reason: String,
    },
}
