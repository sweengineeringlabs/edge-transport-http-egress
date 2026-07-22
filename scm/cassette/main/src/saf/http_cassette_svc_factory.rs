//! Composition site for [`HttpCassette`] — one file per trait keeps wiring
//! focused.
//!
//! `from_layer` is intentionally not paired with an `impl HttpCassette for
//! HttpCassetteFactory` here: `HttpCassetteFactory` is a zero-state unit
//! struct with nothing to report a `mode` for on its own — a delegating
//! impl would have to fabricate a config or construct a throwaway layer,
//! which would be misleading if ever used as a `Box<dyn HttpCassette>`
//! directly instead of through `from_layer`.

use crate::api::{CassetteLayer, HttpCassette};

/// Factory that exposes an existing [`CassetteLayer`]'s mode-inspection surface.
pub struct HttpCassetteFactory;

impl HttpCassetteFactory {
    /// Upcast an existing [`CassetteLayer`] to its [`HttpCassette`] trait
    /// object — the operating mode is a property of a live layer instance,
    /// not something constructed standalone.
    pub fn from_layer(layer: CassetteLayer) -> Box<dyn HttpCassette> {
        Box::new(layer)
    }
}
