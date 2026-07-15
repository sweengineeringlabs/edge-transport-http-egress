//! HTTP response type.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An HTTP response.
///
/// Returned by [`HttpEgress::send`](crate::HttpEgress::send). Use
/// `is_success()`, `is_client_error()`, and `is_server_error()` to classify
/// the status without comparing raw numbers. Call `json::<T>()` to deserialize
/// the body or `text()` to decode it as UTF-8.
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_transport::HttpResponse;
///
/// let resp = HttpResponse::new(200, b"{\"id\": 1}".to_vec());
/// assert!(resp.is_success());
/// assert!(!resp.is_client_error());
/// assert!(!resp.is_server_error());
///
/// let resp_404 = HttpResponse::new(404, b"not found".to_vec());
/// assert!(resp_404.is_client_error());
///
/// let resp_503 = HttpResponse::new(503, vec![]);
/// assert!(resp_503.is_server_error());
///
/// // Decode body as UTF-8 text.
/// let resp = HttpResponse::new(200, b"hello".to_vec());
/// assert_eq!(resp.text().unwrap(), "hello");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response headers.
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// Response body bytes.
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Construct a response with the given status and body; headers default to empty.
    pub fn new(status: u16, body: Vec<u8>) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body,
        }
    }

    /// Returns `true` for 2xx status codes.
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Returns `true` for 4xx status codes.
    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    /// Returns `true` for 5xx status codes.
    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }

    /// Deserialise the body as JSON.
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }

    /// Decode the body as UTF-8 text.
    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }

    /// Look up a response header (RFC 7230 case-insensitive: exact → lowercase → full scan).
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
}
