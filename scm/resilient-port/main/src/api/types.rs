//! Request DTOs for [`crate::ResilientLayers`].

use reqwest_middleware::ClientBuilder;
use swe_edge_configbuilder::SectionLoaderImpl;

use edge_transport_http_egress_cassette::CassetteConfig;

/// Request to apply the resilient chain using config-section-driven
/// activation — each of `[retry]`/`[rate]`/`[breaker]`/`[cache]`/
/// `[cassette]` is wired **iff** its section is present in `loader`;
/// absent (or `enabled = false`) omits it from the chain entirely, not as
/// a no-op layer.
pub struct ApplyFromConfigRequest<'a> {
    /// The builder to decorate. Callers own everything added before this
    /// call (e.g. a caller-supplied auth layer) — this chain is appended
    /// after whatever's already registered.
    pub builder: ClientBuilder,
    /// Config-section source for the five layers.
    pub loader: &'a SectionLoaderImpl,
}

/// Request to apply the resilient chain using SWE-shipped defaults for
/// retry/rate/breaker/cache, unconditionally — no config source. Cassette
/// is the one layer whose activation the caller still controls directly
/// (recording semantics differ meaningfully by use case — see
/// [`CassetteConfig::default`] vs [`CassetteConfig::disabled`]), so it
/// takes an explicit config and name rather than an implicit default.
pub struct ApplyDefaultsRequest {
    /// The builder to decorate.
    pub builder: ClientBuilder,
    /// Cassette configuration — pass [`CassetteConfig::disabled`] to omit
    /// recording entirely.
    pub cassette: CassetteConfig,
    /// Cassette fixture name, used to key the recording file.
    pub cassette_name: String,
}
