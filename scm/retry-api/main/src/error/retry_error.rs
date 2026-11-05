//! Error type for the retry middleware.

/// Errors raised by the retry middleware.
#[derive(Debug, thiserror::Error)]
pub enum RetryError {
    /// Config TOML didn't parse as the expected schema.
    /// Wraps the underlying `toml::de::Error` message, which
    /// names the missing or unknown field when that's the cause.
    #[error("edge_transport_http_egress_retry: config parse failed — {0}")]
    ParseFailed(String),

    /// A configured value is outside its valid range
    /// (e.g. `multiplier <= 0.0`, or `max_interval_ms < initial_interval_ms`).
    #[error("edge_transport_http_egress_retry: invalid config — {0}")]
    InvalidConfig(String),
}
