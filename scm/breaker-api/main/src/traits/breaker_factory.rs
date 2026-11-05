//! The construction contract of the circuit-breaker middleware.

use std::sync::Arc;

use crate::{BreakerConfig, BreakerError, BreakerLayer, Processor, Validator};

/// Builds every consumer-facing breaker object, so no implementation type is public.
pub trait BreakerFactory: Send + Sync {
    /// A [`Processor`] that describes this middleware.
    fn create_processor(&self) -> Box<dyn Processor>;

    /// A [`Validator`] that checks a [`BreakerConfig`].
    fn create_validator(&self) -> Box<dyn Validator>;

    /// A configbuilder seeded with this package's name and version.
    fn create_config_builder(&self) -> configbuilder::ConfigBuilderImpl;

    /// Build the per-host breaker for `config`.
    ///
    /// # Errors
    /// Returns a [`BreakerError`] when the layer cannot be built from `config`.
    fn build_breaker_layer(
        &self,
        config: BreakerConfig,
    ) -> Result<Arc<dyn BreakerLayer>, BreakerError>;
}
