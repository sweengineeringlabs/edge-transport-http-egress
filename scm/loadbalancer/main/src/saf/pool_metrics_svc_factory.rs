//! Composition site for [`PoolMetrics`] — one file per trait keeps wiring
//! focused.
//!
//! `from_layer` is intentionally not paired with an `impl PoolMetrics for
//! PoolMetricsFactory` here: `PoolMetricsFactory` is a zero-state unit
//! struct with nothing to report a `backend_count` for on its own — a
//! delegating impl would have to fabricate a config or construct a
//! throwaway layer, which would be misleading if ever used as a
//! `Box<dyn PoolMetrics>` directly instead of through `from_layer`.

use crate::api::{LoadbalancerLayerPoolMetrics, PoolMetrics};

/// Factory that exposes an existing [`LoadbalancerLayerPoolMetrics`]'s
/// pool-inspection surface.
pub struct PoolMetricsFactory;

impl PoolMetricsFactory {
    /// Upcast an existing [`LoadbalancerLayerPoolMetrics`] to its
    /// [`PoolMetrics`] trait object — the backend count is a property of a
    /// live layer instance, not something constructed standalone.
    pub fn from_layer(layer: LoadbalancerLayerPoolMetrics) -> Box<dyn PoolMetrics> {
        Box::new(layer)
    }
}
