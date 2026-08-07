//! Fluent builder for [`RetryConfig`] — declaration only.
//!
//! The constructor, setters, and `build` method live in `core/`.

/// Fluent builder for [`RetryConfig`](crate::api::RetryConfig).
///
/// Construct via `RetryConfigBuilder::new`, set fields, then call
/// `RetryConfigBuilder::build`. The impl lives in `core/`.
pub struct RetryConfigBuilder {
    pub(crate) max_retries: u32,
    pub(crate) initial_interval_ms: u64,
    pub(crate) max_interval_ms: u64,
    pub(crate) multiplier: f64,
    pub(crate) jitter_factor: f64,
    pub(crate) retryable_statuses: Vec<u16>,
    pub(crate) retryable_methods: Vec<String>,
}
