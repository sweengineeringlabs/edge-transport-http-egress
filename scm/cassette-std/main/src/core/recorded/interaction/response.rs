//! Serializable shape of a recorded HTTP response.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The response half of a recorded HTTP exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RecordedResponse {
    pub(crate) status: u16,
    /// Response headers. BTreeMap so the on-disk serialization
    /// is stable across runs (reproducible diffs).
    pub(crate) headers: BTreeMap<String, String>,
    /// Base64-encoded body bytes — survives binary content.
    pub(crate) body_base64: String,
}
