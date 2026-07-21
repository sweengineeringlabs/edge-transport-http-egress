//! Composition site for [`HttpCache`] — one file per trait keeps wiring
//! focused.
//!
//! `from_layer` is intentionally not paired with an `impl HttpCache for
//! HttpCacheSvcFactory` here: `HttpCacheSvcFactory` is a zero-state unit struct
//! with nothing to report a `default_ttl` for on its own — a delegating
//! impl would have to fabricate a config or construct a throwaway layer,
//! which would be misleading if ever used as a `Box<dyn HttpCache>` directly
//! instead of through `from_layer`.

use crate::api::{HttpCache, MiddlewareHttpCache};

/// Factory that exposes an existing [`MiddlewareHttpCache`]'s policy-inspection surface.
pub struct HttpCacheSvcFactory;

impl HttpCacheSvcFactory {
    /// Upcast an existing [`MiddlewareHttpCache`] to its [`HttpCache`] trait object —
    /// the resolved policy is a property of a live layer instance, not
    /// something constructed standalone.
    pub fn from_layer(layer: MiddlewareHttpCache) -> Box<dyn HttpCache> {
        Box::new(layer)
    }
}
