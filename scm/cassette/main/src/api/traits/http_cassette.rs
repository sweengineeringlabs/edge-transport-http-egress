//! `HttpCassette` — record/replay contract for the cassette middleware layer.

use crate::api::{CassetteError, CassetteModeRequest, CassetteModeResponse};

/// Record/replay contract implemented by the cassette middleware layer.
pub trait HttpCassette: Send + Sync {
    /// Return the operating mode the layer was built with.
    fn mode(&self, request: CassetteModeRequest) -> Result<CassetteModeResponse, CassetteError>;
}
