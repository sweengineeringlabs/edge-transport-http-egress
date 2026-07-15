//! HTTP outbound error type.

use thiserror::Error;

/// Error type for HTTP outbound operations.
///
/// Returned by [`HttpEgress::send`](crate::HttpEgress::send). Match on
/// the variant to apply different recovery strategies: retry on `Timeout` and
/// `ServiceUnavailable`, return 401 to the caller on `Unauthorized`, etc.
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_transport::HttpEgressError;
///
/// fn http_status(e: &HttpEgressError) -> u16 {
///     match e {
///         HttpEgressError::Unauthorized(_)       => 401,
///         HttpEgressError::Forbidden(_)          => 403,
///         HttpEgressError::NotFound(_)           => 404,
///         HttpEgressError::RateLimited(_)        => 429,
///         HttpEgressError::BadGateway(_)         => 502,
///         HttpEgressError::ServiceUnavailable(_) => 503,
///         HttpEgressError::Timeout(_)            => 504,
///         HttpEgressError::ConnectionFailed(_)
///         | HttpEgressError::InvalidRequest(_)
///         | HttpEgressError::Internal(_)         => 500,
///     }
/// }
///
/// let err = HttpEgressError::Timeout("30s deadline exceeded".to_string());
/// assert_eq!(http_status(&err), 504);
/// assert!(err.to_string().contains("30s"));
/// ```
#[derive(Debug, Error)]
pub enum HttpEgressError {
    /// Transport-level connection failure.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
    /// Deadline elapsed before a response was received.
    #[error("timeout: {0}")]
    Timeout(String),
    /// The outbound request was malformed.
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    /// Unexpected client-side error.
    #[error("internal: {0}")]
    Internal(String),
    /// Remote returned HTTP 401 — caller not authenticated.
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    /// Remote returned HTTP 403 — caller lacks permission.
    #[error("forbidden: {0}")]
    Forbidden(String),
    /// Remote returned HTTP 404.
    #[error("not found: {0}")]
    NotFound(String),
    /// Remote returned HTTP 429 — rate limit exceeded.
    #[error("rate limited: {0}")]
    RateLimited(String),
    /// Remote returned HTTP 502 — upstream gateway error.
    #[error("bad gateway: {0}")]
    BadGateway(String),
    /// Remote returned HTTP 503.
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
}
