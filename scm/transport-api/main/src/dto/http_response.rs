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
