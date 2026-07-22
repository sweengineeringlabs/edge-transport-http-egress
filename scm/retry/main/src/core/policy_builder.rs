//! `impl` blocks for [`RetryConfigBuilder`]. The declaration lives in `api/`.

use crate::api::RetryConfig;
use crate::api::RetryConfigBuilder;

impl RetryConfigBuilder {
    /// Create a builder pre-populated with the SWE defaults.
    pub fn new() -> Self {
        let defaults = RetryConfig::default();
        Self {
            max_retries: defaults.max_retries,
            initial_interval_ms: defaults.initial_interval_ms,
            max_interval_ms: defaults.max_interval_ms,
            multiplier: defaults.multiplier,
            retryable_statuses: defaults.retryable_statuses,
            retryable_methods: defaults.retryable_methods,
        }
    }

    /// Set the maximum number of retry attempts.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Set the initial backoff interval in milliseconds.
    pub fn initial_interval_ms(mut self, ms: u64) -> Self {
        self.initial_interval_ms = ms;
        self
    }

    /// Set the maximum backoff interval in milliseconds.
    pub fn max_interval_ms(mut self, ms: u64) -> Self {
        self.max_interval_ms = ms;
        self
    }

    /// Set the exponential backoff multiplier.
    pub fn multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = multiplier;
        self
    }

    /// Set the HTTP status codes that trigger a retry.
    pub fn retryable_statuses(mut self, statuses: Vec<u16>) -> Self {
        self.retryable_statuses = statuses;
        self
    }

    /// Set the HTTP methods that can safely be retried.
    pub fn retryable_methods(mut self, methods: Vec<String>) -> Self {
        self.retryable_methods = methods;
        self
    }

    /// Build the [`RetryConfig`], returning `Err` if validation fails
    /// (e.g. `multiplier <= 0`).
    pub fn build(self) -> Result<RetryConfig, String> {
        let config = RetryConfig {
            max_retries: self.max_retries,
            initial_interval_ms: self.initial_interval_ms,
            max_interval_ms: self.max_interval_ms,
            multiplier: self.multiplier,
            retryable_statuses: self.retryable_statuses,
            retryable_methods: self.retryable_methods,
        };
        config.validate().map(|()| config)
    }
}

impl Default for RetryConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
