//! HTTP request type.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::http_body::HttpBody;
use super::http_method::HttpMethod;

/// An HTTP request.
///
/// Convenience constructors (`get`, `post`, `put`, `delete`, `patch`, `head`,
/// `options`) create a request with no body and no headers. Chain the `with_*`
/// methods to add headers, query params, a body, and a timeout.
///
/// # Examples
///
/// ```rust
/// use std::time::Duration;
/// use edge_transport_http_egress_transport::{HttpMethod, HttpRequest};
///
/// // Quick GET.
/// let req = HttpRequest::get("https://api.example.com/users");
/// assert_eq!(req.method, HttpMethod::Get);
/// assert_eq!(req.url, "https://api.example.com/users");
/// assert!(req.body.is_none());
///
/// // POST with JSON body.
/// let req = HttpRequest::post("https://api.example.com/users")
///     .with_header("x-request-id", "abc-123")
///     .with_query("include", "roles")
///     .with_json(&serde_json::json!({ "name": "Alice" }))
///     .unwrap()
///     .with_timeout(Duration::from_secs(10));
///
/// assert_eq!(req.url, "https://api.example.com/users");
/// assert_eq!(req.header("x-request-id"), Some("abc-123"));
/// assert_eq!(req.timeout, Some(Duration::from_secs(10)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Full URL or path relative to the client's base URL.
    pub url: String,
    /// Request headers (merged with client-level defaults at send time).
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// URL query parameters appended at send time.
    #[serde(default)]
    pub query: HashMap<String, String>,
    /// Optional request body.
    pub body: Option<HttpBody>,
    /// Per-request timeout override (uses client default when `None`).
    pub timeout: Option<Duration>,
}

impl HttpRequest {
    /// Build a GET request.
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Get,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Build a POST request.
    pub fn post(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Post,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Build a PUT request.
    pub fn put(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Put,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Build a DELETE request.
    pub fn delete(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Delete,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Build a PATCH request.
    pub fn patch(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Patch,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Build a HEAD request.
    pub fn head(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Head,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Build an OPTIONS request.
    pub fn options(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Options,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Look up a request header (RFC 7230 case-insensitive: exact → lowercase → full scan).
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .get(name)
            .or_else(|| self.headers.get(&name.to_lowercase()))
            .map(String::as_str)
            .or_else(|| {
                self.headers
                    .iter()
                    .find(|(k, _)| k.eq_ignore_ascii_case(name))
                    .map(|(_, v)| v.as_str())
            })
    }

    /// Add a request header.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Add a URL query parameter.
    pub fn with_query(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(name.into(), value.into());
        self
    }

    /// Set a JSON body and `Content-Type: application/json`.
    pub fn with_json<T: Serialize>(mut self, body: &T) -> Result<Self, serde_json::Error> {
        self.body = Some(HttpBody::Json(serde_json::to_value(body)?));
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        Ok(self)
    }

    /// Set a raw byte body and the given `Content-Type`.
    pub fn with_body(mut self, body: Vec<u8>, content_type: impl Into<String>) -> Self {
        self.body = Some(HttpBody::Raw(body));
        self.headers
            .insert("Content-Type".to_string(), content_type.into());
        self
    }

    /// Set a form-encoded body and `Content-Type: application/x-www-form-urlencoded`.
    pub fn with_form(mut self, form: HashMap<String, String>) -> Self {
        self.body = Some(HttpBody::Form(form));
        self.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );
        self
    }

    /// Set a per-request timeout, overriding the client-level default.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}
