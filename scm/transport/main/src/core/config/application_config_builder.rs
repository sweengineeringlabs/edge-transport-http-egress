//! `impl ApplicationConfigBuilder` — the declaration lives in `api/`.

use crate::api::ApplicationConfigBuilder;

impl ApplicationConfigBuilder {
    /// Create a new, unseeded builder.
    pub fn new() -> Self {
        Self(swe_edge_configbuilder::ConfigBuilderImpl::new())
    }

    /// The package name this builder was seeded with.
    pub fn name(&self) -> &str {
        self.0.name()
    }

    /// The package version this builder was seeded with.
    pub fn version(&self) -> &str {
        self.0.version()
    }

    /// Set the package name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.0 = self.0.with_name(name);
        self
    }

    /// Set the package version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.0 = self.0.with_version(version);
        self
    }

    /// Consume the builder and return a ready-to-use section loader.
    pub fn build_loader(
        self,
    ) -> Result<swe_edge_configbuilder::SectionLoaderImpl, swe_edge_configbuilder::ConfigError>
    {
        self.0.build_loader()
    }
}

impl Default for ApplicationConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
