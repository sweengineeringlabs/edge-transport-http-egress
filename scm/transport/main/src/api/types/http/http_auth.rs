//! HTTP authentication types.

use serde::{Deserialize, Serialize};

/// HTTP authentication scheme.
///
/// Used in [`HttpRequest`] to carry credentials alongside the request. The
/// transport layer converts this to the appropriate `Authorization` or custom
/// header before sending.
///
/// [`HttpRequest`]: super::http_request::HttpRequest
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_transport::HttpAuth;
///
/// let bearer = HttpAuth::bearer("my-token");
/// assert!(matches!(bearer, HttpAuth::Bearer { .. }));
///
/// let basic = HttpAuth::basic("alice", "secret");
/// assert!(matches!(basic, HttpAuth::Basic { .. }));
///
/// let api_key = HttpAuth::api_key("x-api-key", "key-value");
/// if let HttpAuth::ApiKey { header, key } = api_key {
///     assert_eq!(header, "x-api-key");
///     assert_eq!(key, "key-value");
/// }
///
/// let none = HttpAuth::None;
/// assert!(matches!(none, HttpAuth::None));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HttpAuth {
    /// No authentication — requests are sent without credentials.
    None,
    /// `Authorization: Bearer <token>`.
    Bearer {
        /// The bearer token value.
        token: String,
    },
    /// `Authorization: Basic base64(username:password)`.
    Basic {
        /// Username component.
        username: String,
        /// Password component.
        password: String,
    },
    /// Custom credential header (e.g. `x-api-key`).
    ApiKey {
        /// Header name.
        header: String,
        /// Credential value.
        key: String,
    },
}

impl HttpAuth {
    /// Construct a `Bearer` variant.
    pub fn bearer(token: impl Into<String>) -> Self {
        HttpAuth::Bearer {
            token: token.into(),
        }
    }

    /// Construct a `Basic` variant.
    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Self {
        HttpAuth::Basic {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Construct an `ApiKey` variant.
    pub fn api_key(header: impl Into<String>, key: impl Into<String>) -> Self {
        HttpAuth::ApiKey {
            header: header.into(),
            key: key.into(),
        }
    }
}
