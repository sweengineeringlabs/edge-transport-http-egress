//! Local newtype wrapping a JSON body value (egress).

use serde::{Deserialize, Serialize};

/// A JSON value carried by [`HttpBody::Json`](super::http_body::HttpBody::Json).
///
/// Wraps `serde_json::Value` so `HttpBody`'s variant payload never names the
/// foreign type directly (SEA `no_foreign_type`). Construct with
/// [`JsonValue::new`] and unwrap with [`into_inner`](JsonValue::into_inner).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonValue(pub(crate) serde_json::Value);
