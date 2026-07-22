//! Response for [`crate::api::HttpCassette::mode`].

use serde::{Deserialize, Serialize};

/// Output of [`crate::api::HttpCassette::mode`] — the operating mode
/// (`"replay"` | `"record"` | `"auto"` | `"disabled"`) the layer was built with.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CassetteModeResponse {
    /// The configured operating mode.
    pub value: String,
}
