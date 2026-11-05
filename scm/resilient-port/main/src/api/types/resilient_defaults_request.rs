//! Request data for applying resilient layers with shipped defaults.

use reqwest_middleware::ClientBuilder;

use edge_transport_http_egress_cassette::CassetteConfig;

/// Request to apply the resilient chain using SWE-shipped defaults for
/// retry/rate/breaker/cache, unconditionally — no config source. Cassette
/// is the one layer whose activation the caller still controls directly
/// (recording semantics differ meaningfully by use case — see
/// [`CassetteConfig::default`] vs [`CassetteConfig::disabled`]), so it
/// takes an explicit config and name rather than an implicit default.
pub struct ResilientDefaultsRequest {
    /// The builder to decorate.
    pub builder: ClientBuilder,
    /// Cassette configuration — pass [`CassetteConfig::disabled`] to omit
    /// recording entirely.
    pub cassette: CassetteConfig,
    /// Cassette fixture name, used to key the recording file.
    pub cassette_name: String,
}
