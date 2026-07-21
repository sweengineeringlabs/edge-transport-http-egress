//! `impl JsonValue` — the declaration lives in `api/`.

use crate::api::JsonValue;

impl JsonValue {
    /// Wrap a `serde_json::Value`.
    pub fn new(value: serde_json::Value) -> Self {
        Self(value)
    }

    /// Unwrap back into a real `serde_json::Value`.
    pub fn into_inner(self) -> serde_json::Value {
        self.0
    }
}
