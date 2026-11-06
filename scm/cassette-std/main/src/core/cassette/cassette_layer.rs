//! Public type — the VCR-style cassette middleware.

use std::path::PathBuf;
use std::sync::Arc;

use edge_transport_http_egress_cassette_api::CassetteConfig;

/// Cassette middleware. Attach to a
/// `reqwest_middleware::ClientBuilder` via `.with(layer)`.
///
/// Modes:
/// - `"replay"`: read-only — replay fixtures; fail on cache miss
/// - `"record"`: always hit upstream; overwrite fixture on every
///   request (including subsequent to re-record stale data)
/// - `"auto"`: replay on hit; record on miss (local dev default)
pub(crate) struct FileCassetteLayer {
    pub(crate) config: Arc<CassetteConfig>,
    pub(crate) cassette_path: PathBuf,
    pub(crate) fixtures: Arc<crate::core::cassette::fixture_store::FixtureStore>,
}
