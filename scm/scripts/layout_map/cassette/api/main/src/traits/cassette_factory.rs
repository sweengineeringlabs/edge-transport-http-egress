//! The construction contract of the cassette middleware.

use std::sync::Arc;

use crate::{CassetteConfig, CassetteError, CassetteLayer, Processor, Validator};

/// Builds every consumer-facing cassette object, so no implementation type is public.
pub trait CassetteFactory: Send + Sync {
    /// A [`Processor`] that describes this middleware.
    fn create_processor(&self) -> Box<dyn Processor>;

    /// A [`Validator`] that checks a [`CassetteConfig`].
    fn create_validator(&self) -> Box<dyn Validator>;

    /// A configbuilder seeded with the application name of this middleware and the package version.
    fn create_config_builder(&self) -> configbuilder::ConfigBuilderImpl;

    /// Build the record/replay layer for `config`, bound to the fixture file
    /// `<cassette_dir>/<cassette_name>.yaml`.
    ///
    /// # Errors
    /// Returns a [`CassetteError`] when the existing fixture file cannot be read or parsed.
    fn build_cassette_layer(
        &self,
        config: CassetteConfig,
        cassette_name: &str,
    ) -> Result<Arc<dyn CassetteLayer>, CassetteError>;
}
