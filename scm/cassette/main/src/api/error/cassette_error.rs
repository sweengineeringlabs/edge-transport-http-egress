//! `CassetteError` — domain error for the cassette middleware.

/// Errors raised by the cassette middleware.
#[derive(Debug, thiserror::Error)]
pub enum CassetteError {
    /// Config TOML didn't parse as the expected schema.
    #[error("edge_transport_http_egress_cassette: config parse failed — {0}")]
    ParseFailed(String),

    /// A `CassetteConfig` field failed structural validation.
    #[error("edge_transport_http_egress_cassette: invalid config — {0}")]
    InvalidConfig(String),
}
