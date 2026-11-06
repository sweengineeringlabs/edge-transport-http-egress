//! The construction contract of the retry middleware.

use crate::{ApplicationConfigBuilder, Processor, RetryConfigBuilder, Validator};

/// Builds every consumer-facing retry object, so no implementation type is public.
///
/// The retry layer itself is built by [`Processor::decorate`] on the processor this creates.
pub trait RetryFactory: Send + Sync {
    /// A [`Processor`] that describes this middleware and decorates a config into a layer.
    fn create_processor(&self) -> Box<dyn Processor>;

    /// A [`Validator`] that checks a [`RetryConfig`](crate::RetryConfig).
    fn create_validator(&self) -> Box<dyn Validator>;

    /// A fresh builder for a [`RetryConfig`](crate::RetryConfig).
    fn new_config_builder(&self) -> RetryConfigBuilder;

    /// A builder for the application config loader, seeded with this middleware name and version.
    fn new_app_config_builder(&self) -> ApplicationConfigBuilder;
}
