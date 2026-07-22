//! `impl HttpRequestBuilder` — the declaration lives in `api/`.

use std::collections::HashMap;
use std::time::Duration;

use crate::api::{HttpAuth, HttpMethod, HttpRequest, HttpRequestBuilder};

impl HttpRequestBuilder {
    /// Create a new builder for the given HTTP method and URL.
    pub fn new(method: HttpMethod, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
            auth: None,
        }
    }

    /// Add a request header.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Add a query parameter.
    pub fn with_query(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(name.into(), value.into());
        self
    }

    /// Set the request timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Attach credentials, applied as the `Authorization` (or custom) header
    /// at send time.
    pub fn with_auth(mut self, auth: HttpAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Consume the builder and return the configured [`HttpRequest`].
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            headers: self.headers,
            query: self.query,
            body: self.body,
            timeout: self.timeout,
            auth: self.auth,
        }
    }
}
