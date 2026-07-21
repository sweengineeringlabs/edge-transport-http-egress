//! Error type for the breaker middleware.

/// Errors raised by the breaker middleware.
#[derive(Debug, thiserror::Error)]
pub enum BreakerError {
    /// Config TOML didn't parse as the expected schema.
    #[error("edge_transport_http_egress_breaker: config parse failed — {0}")]
    ParseFailed(String),

    /// The circuit for `host` is open — the request was rejected without
    /// being sent.  Callers can downcast from `reqwest_middleware::Error`
    /// to inspect this variant and apply a fallback.
    #[error("edge_transport_http_egress_breaker: circuit open for '{host}' — request rejected")]
    CircuitOpen {
        /// The host key for which the circuit tripped.
        host: String,
    },

    /// A `BreakerConfig` field failed structural validation.
    #[error("edge_transport_http_egress_breaker: invalid config — {0}")]
    InvalidConfig(String),
}
