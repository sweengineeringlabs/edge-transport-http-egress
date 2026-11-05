//! Request data for applying resilient layers from config sections.

use configbuilder::SectionLoaderImpl;
use reqwest_middleware::ClientBuilder;

/// Request to apply the resilient chain using config-section-driven
/// activation — each of `[retry]`/`[rate]`/`[breaker]`/`[cache]`/
/// `[cassette]` is wired **iff** its section is present in `loader`;
/// absent (or `enabled = false`) omits it from the chain entirely, not as
/// a no-op layer.
pub struct ResilientConfigRequest<'a> {
    /// The builder to decorate. Callers own everything added before this
    /// call (e.g. a caller-supplied auth layer) — this chain is appended
    /// after whatever's already registered.
    pub builder: ClientBuilder,
    /// Config-section source for the five layers.
    pub loader: &'a SectionLoaderImpl,
}
