//! Fluent builder for [`HttpRequest`].

use std::collections::HashMap;
use std::time::Duration;

use super::http_body::HttpBody;
use super::http_method::HttpMethod;
use super::http_request::HttpRequest;

/// Fluent builder for [`HttpRequest`].
///
/// Construct via [`HttpRequestBuilder::new`] and chain setter methods,
/// then call [`build`](Self::build) to obtain the final [`HttpRequest`].
///
/// # Examples
///
/// ```rust
/// use std::time::Duration;
/// use edge_transport_http_egress_transport::{HttpMethod, HttpRequestBuilder};
///
/// let req = HttpRequestBuilder::new(HttpMethod::Get, "https://api.example.com/items")
///     .with_header("Authorization", "Bearer my-token")
///     .with_query("page", "2")
///     .with_timeout(Duration::from_secs(30))
///     .build();
///
/// assert_eq!(req.url, "https://api.example.com/items");
/// assert_eq!(req.headers.get("Authorization").map(String::as_str), Some("Bearer my-token"));
/// assert_eq!(req.query.get("page").map(String::as_str), Some("2"));
/// assert_eq!(req.timeout, Some(Duration::from_secs(30)));
/// assert!(req.body.is_none());
/// ```
#[derive(Debug)]
pub struct HttpRequestBuilder {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    query: HashMap<String, String>,
    body: Option<HttpBody>,
    timeout: Option<Duration>,
}

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

    /// Consume the builder and return the configured [`HttpRequest`].
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            headers: self.headers,
            query: self.query,
            body: self.body,
            timeout: self.timeout,
        }
    }
}
