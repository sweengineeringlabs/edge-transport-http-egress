//! The construction contract of the HTTP-cache middleware.

use std::sync::Arc;

use crate::{CacheConfig, CacheError, CacheLayer, Processor, Validator};

/// Builds every consumer-facing cache object, so no implementation type is public.
pub trait CacheFactory: Send + Sync {
    /// A [`Processor`] that describes this middleware.
    fn create_processor(&self) -> Box<dyn Processor>;

    /// A [`Validator`] that checks a [`CacheConfig`].
    fn create_validator(&self) -> Box<dyn Validator>;

    /// A configbuilder seeded with the application name of this middleware and the package version.
    fn create_config_builder(&self) -> configbuilder::ConfigBuilderImpl;

    /// Build the HTTP cache for `config`.
    ///
    /// # Errors
    /// Returns a [`CacheError`] when the cache cannot be built from `config`.
    fn build_cache_layer(&self, config: CacheConfig) -> Result<Arc<dyn CacheLayer>, CacheError>;
}
