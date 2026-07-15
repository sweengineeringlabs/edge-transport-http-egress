//! `OAuthError` — errors produced by the OAuth crate.

/// Errors produced by the OAuth crate.
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    /// OAuth credentials could not be located.
    #[error("edge_transport_http_egress_oauth: credentials not found: {0}")]
    CredentialsNotFound(String),
    /// Token refresh request failed.
    #[error("edge_transport_http_egress_oauth: token refresh failed: {0}")]
    RefreshFailed(String),
    /// Underlying HTTP request failed.
    #[error("edge_transport_http_egress_oauth: http error: {0}")]
    Http(String),
    /// Invalid or missing configuration.
    #[error("edge_transport_http_egress_oauth: configuration error: {0}")]
    Configuration(String),
}

impl From<OAuthError> for swe_edge_security::SecurityError {
    fn from(err: OAuthError) -> Self {
        swe_edge_security::SecurityError::Token(err.to_string())
    }
}
