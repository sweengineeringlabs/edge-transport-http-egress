//! Fluent builder for [`HttpRequest`].

use std::collections::HashMap;
use std::time::Duration;

use super::http_auth::HttpAuth;
use super::http_method::HttpMethod;
use crate::api::HttpBody;

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
    pub(crate) method: HttpMethod,
    pub(crate) url: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) query: HashMap<String, String>,
    pub(crate) body: Option<HttpBody>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) auth: Option<HttpAuth>,
}
