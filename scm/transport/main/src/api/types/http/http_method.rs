//! HTTP method enum.

use serde::{Deserialize, Serialize};
use std::fmt;

/// An HTTP request method.
///
/// Implements [`Display`] returning the uppercase wire form (`"GET"`, `"POST"`, etc.).
///
/// [`Display`]: std::fmt::Display
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_transport::HttpMethod;
///
/// assert_eq!(HttpMethod::Get.to_string(), "GET");
/// assert_eq!(HttpMethod::Post.to_string(), "POST");
/// assert_eq!(HttpMethod::Delete.to_string(), "DELETE");
/// assert_ne!(HttpMethod::Get, HttpMethod::Post);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    /// HTTP GET.
    Get,
    /// HTTP POST.
    Post,
    /// HTTP PUT.
    Put,
    /// HTTP PATCH.
    Patch,
    /// HTTP DELETE.
    Delete,
    /// HTTP HEAD.
    Head,
    /// HTTP OPTIONS.
    Options,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Head => write!(f, "HEAD"),
            HttpMethod::Options => write!(f, "OPTIONS"),
        }
    }
}
