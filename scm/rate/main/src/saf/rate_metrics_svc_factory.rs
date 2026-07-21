//! Composition site for [`RateMetrics`] — one file per trait keeps wiring
//! focused.
//!
//! `from_layer` is intentionally not paired with an `impl RateMetrics for
//! RateMetricsFactory` here: `RateMetricsFactory` is a zero-state unit struct
//! with no configured rate to report on its own — a delegating impl would have
//! to fabricate a config or construct a throwaway layer, which would be
//! misleading if ever used as a `Box<dyn RateMetrics>` directly instead of
//! through `from_layer`.

use crate::api::{RateLayerRateMetrics, RateMetrics};

/// Factory that exposes an existing [`RateLayerRateMetrics`]'s metrics surface.
pub struct RateMetricsFactory;

impl RateMetricsFactory {
    /// Upcast an existing [`RateLayerRateMetrics`] to its [`RateMetrics`] trait
    /// object — the rate limit is a property of a live layer instance, not
    /// something constructed standalone.
    pub fn from_layer(layer: RateLayerRateMetrics) -> Box<dyn RateMetrics> {
        Box::new(layer)
    }
}
