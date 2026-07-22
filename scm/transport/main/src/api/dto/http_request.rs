//! HTTP request type.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::api::dto::http_body::HttpBody;
use crate::api::types::http::{HttpAuth, HttpMethod};

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
    /// Optional credentials applied as the `Authorization` (or custom) header
    /// at send time. `None` sends the request with no explicit credentials.
    #[serde(default)]
    pub auth: Option<HttpAuth>,
}
