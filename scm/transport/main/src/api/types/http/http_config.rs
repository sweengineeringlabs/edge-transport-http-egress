//! HTTP client configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HTTP client configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HttpConfig {
    /// Optional base URL prepended to relative request URLs.
    pub base_url: Option<String>,
    /// Total request timeout in seconds (default: 30).
    pub timeout_secs: u64,
    /// TCP connection timeout in seconds (default: 10).
    pub connect_timeout_secs: u64,
    /// Maximum retry attempts for transient failures (default: 3).
    pub max_retries: u32,
    /// Headers attached to every outbound request.
    #[serde(default)]
    pub default_headers: HashMap<String, String>,
    /// Whether to follow HTTP 3xx redirects (default: true).
    pub follow_redirects: bool,
    /// Maximum number of redirects to follow (default: 10).
    pub max_redirects: u32,
    /// `User-Agent` header value (default: `"swe-edge/0.1.0"`).
    pub user_agent: Option<String>,
    /// Maximum response body size in bytes; `None` disables the cap (default: 10 MiB).
    #[serde(default = "HttpConfig::default_max_response_bytes")]
    pub max_response_bytes: Option<usize>,
}
