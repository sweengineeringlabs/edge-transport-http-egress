//! `ResilientError` — the structured error taxonomy for [`crate::ResilientLayers`].

use thiserror::Error;

/// Errors that can occur while composing the retry/rate/breaker/cache/
/// cassette chain onto a [`reqwest_middleware::ClientBuilder`].
///
/// One variant per layer that can fail to build, plus [`Config`](ResilientError::Config)
/// for config-section load/validation failures — mirrors the shape
/// `HttpEgressBuildError` used before this chain lived in `transport`.
#[derive(Debug, Error)]
pub enum ResilientError {
    /// A config section failed to load or validate.
    #[error("config error: {0}")]
    Config(String),
    /// The retry layer failed to build.
    #[error("retry layer error: {0}")]
    Retry(String),
    /// The rate layer failed to build.
    #[error("rate layer error: {0}")]
    Rate(String),
    /// The breaker layer failed to build.
    #[error("breaker layer error: {0}")]
    Breaker(String),
    /// The cache layer failed to build.
    #[error("cache layer error: {0}")]
    Cache(String),
    /// The cassette layer failed to build.
    #[error("cassette layer error: {0}")]
    Cassette(String),
}
