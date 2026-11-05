//! The construction contract of the rate-limit middleware.

use std::sync::Arc;

use crate::{Processor, RateConfig, RateError, RateLayer, Validator};

/// Builds every consumer-facing rate-limit object, so no implementation type is public.
pub trait RateFactory: Send + Sync {
    /// A [`Processor`] that describes this middleware.
    fn create_processor(&self) -> Box<dyn Processor>;

    /// A [`Validator`] that checks a [`RateConfig`].
    fn create_validator(&self) -> Box<dyn Validator>;

    /// A configbuilder seeded with this package's name and version.
    fn create_config_builder(&self) -> configbuilder::ConfigBuilderImpl;

    /// Build the per-host rate limiter for `config`.
    ///
    /// # Errors
    /// Returns a [`RateError`] when `config` is invalid.
    fn build_rate_layer(&self, config: RateConfig) -> Result<Arc<dyn RateLayer>, RateError>;
}
