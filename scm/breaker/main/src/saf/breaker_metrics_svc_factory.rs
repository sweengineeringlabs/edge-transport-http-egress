//! Composition site for [`BreakerMetrics`] — one file per trait keeps
//! wiring focused.
//!
//! `from_layer` is intentionally not paired with an `impl BreakerMetrics for
//! BreakerMetricsFactory` here: `BreakerMetricsFactory` is a zero-state unit
//! struct with nothing to report a `failure_threshold` for on its own — a
//! delegating impl would have to fabricate a config or construct a throwaway
//! layer, which would be misleading if ever used as a `Box<dyn BreakerMetrics>`
//! directly instead of through `from_layer`.

use crate::api::{BreakerLayerBreakerMetrics, BreakerMetrics};

/// Factory that exposes an existing [`BreakerLayerBreakerMetrics`]'s metrics surface.
pub struct BreakerMetricsFactory;

impl BreakerMetricsFactory {
    /// Upcast an existing [`BreakerLayerBreakerMetrics`] to its [`BreakerMetrics`] trait
    /// object — metrics are a property of a live layer instance, not
    /// something constructed standalone.
    pub fn from_layer(layer: BreakerLayerBreakerMetrics) -> Box<dyn BreakerMetrics> {
        Box::new(layer)
    }
}
