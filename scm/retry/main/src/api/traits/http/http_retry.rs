//! Primary trait for the retry crate.
//!
//! Crate-level processor contract. Rule 153 requires the
//! primary trait to match the declared service_type (=
//! "processor"); we use the domain-prefixed form `HttpRetry`
//! for clarity at use sites. The impl lives in
//! `core/default_http_retry`.

use crate::api::types::retry::retry_config::RetryConfig;

/// The retry crate's primary trait. Every middleware layer
/// produced by this crate implements it.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "SEA api/ interface anchor — exercised only via tests"
    )
)]
pub trait HttpRetry: Send + Sync {
    /// Identify this processor in log / trace output.
    ///
    /// Returns the crate's canonical name (e.g. `"edge_transport_http_egress_retry"`).
    fn describe(&self) -> &'static str;

    /// Return the underlying retry configuration.
    fn config(&self) -> &RetryConfig;
}
