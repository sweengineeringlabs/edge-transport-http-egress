//! `impl HttpConfigBuilder` — the declaration lives in `api/`.

use crate::api::{HttpConfig, HttpConfigBuilder};

impl HttpConfigBuilder {
    /// Create a new builder with all fields unset (uses [`HttpConfig`] defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL for the HTTP client.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the request timeout in seconds.
    pub fn with_timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    /// Set the connection timeout in seconds.
    pub fn with_connect_timeout_secs(mut self, secs: u64) -> Self {
        self.connect_timeout_secs = Some(secs);
        self
    }

    /// Add a default header sent on every request.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(name.into(), value.into());
        self
    }

    /// Set the user agent string.
    pub fn with_user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Consume the builder and return the configured [`HttpConfig`].
    pub fn build(self) -> HttpConfig {
        let base = HttpConfig::default();
        HttpConfig {
            base_url: self.base_url.or(base.base_url),
            timeout_secs: self.timeout_secs.unwrap_or(base.timeout_secs),
            connect_timeout_secs: self
                .connect_timeout_secs
                .unwrap_or(base.connect_timeout_secs),
            default_headers: if self.default_headers.is_empty() {
                base.default_headers
            } else {
                self.default_headers
            },
            user_agent: self.user_agent.or(base.user_agent),
            follow_redirects: base.follow_redirects,
            max_redirects: base.max_redirects,
            max_response_bytes: base.max_response_bytes,
        }
    }
}
