//! `impl Default for HttpConfig` / `impl HttpConfig` — the declaration lives in `api/`.

use std::collections::HashMap;

use crate::api::HttpConfig;

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            base_url: None,
            timeout_secs: 30,
            connect_timeout_secs: 10,
            max_retries: 3,
            default_headers: HashMap::new(),
            follow_redirects: true,
            max_redirects: 10,
            user_agent: Some("swe-edge/0.1.0".to_string()),
            max_response_bytes: HttpConfig::default_max_response_bytes(),
        }
    }
}

impl HttpConfig {
    /// Default response size cap: 10 MiB.
    ///
    /// Used as the serde default for `max_response_bytes` to prevent
    /// unbounded memory allocation when deserialising large HTTP responses.
    pub fn default_max_response_bytes() -> Option<usize> {
        Some(10 * 1024 * 1024)
    }

    /// Create an [`HttpConfig`] with the given base URL and all other fields at their defaults.
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: Some(base_url.into()),
            ..Default::default()
        }
    }

    /// Add a default header sent on every request.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(name.into(), value.into());
        self
    }

    /// Override the request timeout in seconds.
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}
