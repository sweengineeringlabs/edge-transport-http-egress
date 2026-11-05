//! Error type for the rate middleware.

/// Errors raised by the rate middleware.
#[derive(Debug, thiserror::Error)]
pub enum RateError {
    /// Config TOML didn't parse as the expected schema.
    #[error("edge_transport_http_egress_rate: config parse failed — {0}")]
    ParseFailed(String),

    /// A `RateConfig` field failed structural validation.
    #[error("edge_transport_http_egress_rate: invalid config — {0}")]
    InvalidConfig(String),
}
