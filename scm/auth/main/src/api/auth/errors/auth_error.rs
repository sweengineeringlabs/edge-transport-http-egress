//! Error type for the auth middleware.

/// Errors raised by the auth middleware.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// Config TOML didn't parse as the expected schema.
    #[error("edge_transport_http_egress_auth: config parse failed — {0}")]
    ParseFailed(String),

    /// Config references an env var that isn't set. Includes
    /// the missing var name so operators know what to export.
    /// This fails at `AuthSvc::build_auth_middleware(config)` — the middleware refuses
    /// to construct with a dangling credential reference.
    #[error("edge_transport_http_egress_auth: required env var {name} is not set")]
    MissingEnvVar {
        /// Name of the missing env var.
        name: String,
    },

    /// Unknown or unsupported `kind` in config. The config
    /// schema lists the accepted values.
    #[error("edge_transport_http_egress_auth: unsupported auth kind {kind:?} — expected one of: none, bearer, basic, header")]
    UnsupportedKind {
        /// The offending kind string.
        kind: String,
    },

    /// Credential value can't be encoded as a valid HTTP header
    /// value. Per RFC 7230 header values must be US-ASCII
    /// visible characters + HTAB; CR/LF/NUL are forbidden.
    /// Wraps the underlying parse error for diagnostics.
    #[error("edge_transport_http_egress_auth: credential is not a valid HTTP header value — {0}")]
    InvalidHeaderValue(String),

    /// Config's `name` (for the custom Header scheme) can't be
    /// parsed as a valid HTTP header name. Must be a
    /// token-per-RFC-7230 (alphanumerics + a few symbols).
    #[error("edge_transport_http_egress_auth: invalid header name {name:?} — {reason}")]
    InvalidHeaderName {
        /// The offending name string.
        name: String,
        /// Underlying parse error.
        reason: String,
    },

    /// No credential sources are configured or available.
    /// Raised by CredentialSourceResolver when all sources fail.
    #[error("edge_transport_http_egress_auth: credential resolution failed — {0}")]
    MissingCredential(String),
}
