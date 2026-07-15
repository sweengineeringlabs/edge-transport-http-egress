//! HTTP body variants.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::types::form_part::FormPart;

/// HTTP request body variants.
///
/// The transport layer sets the `Content-Type` header automatically based on
/// the variant: `application/json` for `Json`, `application/octet-stream` for
/// `Raw`, `application/x-www-form-urlencoded` for `Form`, and
/// `multipart/form-data` for `Multipart`.
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_transport::HttpBody;
///
/// let json = HttpBody::Json(serde_json::json!({ "key": "value" }));
/// assert!(matches!(json, HttpBody::Json(_)));
///
/// let raw = HttpBody::Raw(b"binary payload".to_vec());
/// assert!(matches!(raw, HttpBody::Raw(_)));
///
/// let form = HttpBody::Form([("name".to_string(), "Alice".to_string())].into());
/// assert!(matches!(form, HttpBody::Form(_)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HttpBody {
    /// `application/json` body.
    Json(serde_json::Value),
    /// `application/octet-stream` body — arbitrary bytes.
    Raw(Vec<u8>),
    /// `application/x-www-form-urlencoded` body.
    Form(HashMap<String, String>),
    /// `multipart/form-data` body — one or more named parts.
    Multipart(Vec<FormPart>),
}
