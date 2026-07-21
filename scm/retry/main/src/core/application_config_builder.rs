//! `impl` blocks for [`ApplicationConfigBuilder`] — construction and the
//! delegating accessor. The declaration lives in `api/`.

use crate::api::{ApplicationConfigBuilder, RetryError};

impl ApplicationConfigBuilder {
    /// Construct the application config builder for `config/application.toml`,
    /// pre-seeded with this crate's name and version.
    pub(crate) fn new() -> Self {
        let inner = swe_edge_configbuilder::ConfigBuilderImpl::new()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"));
        Self(inner)
    }

    /// The package name the builder was seeded with.
    pub fn name(&self) -> &str {
        self.0.name()
    }

    /// The package version the builder was seeded with.
    pub fn version(&self) -> &str {
        self.0.version()
    }

    /// Point the builder at a specific config directory before building a
    /// loader (defaults to the crate's own shipped `config/` otherwise).
    pub fn with_config_dir(self, dir: impl Into<std::path::PathBuf>) -> Self {
        Self(self.0.with_config_dir(dir))
    }

    /// Build the config loader from the builder's current state.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying loader construction fails (e.g. an
    /// unreadable config directory).
    pub fn build_loader(self) -> Result<swe_edge_configbuilder::SectionLoaderImpl, RetryError> {
        self.0
            .build_loader()
            .map_err(|e| RetryError::InvalidConfig(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: name
    #[test]
    fn test_name_seeds_crate_name() {
        let builder = ApplicationConfigBuilder::new();
        assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
    }

    /// @covers: version
    #[test]
    fn test_version_seeds_crate_version() {
        let builder = ApplicationConfigBuilder::new();
        assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
    }

    /// @covers: build_loader
    #[test]
    fn test_build_loader_loads_the_shipped_retry_section() {
        use crate::api::RetryConfig;

        let loader = ApplicationConfigBuilder::new()
            .with_config_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/config"))
            .build_loader()
            .expect("build_loader must succeed against the shipped config dir");
        let cfg: RetryConfig = loader
            .load_section("retry")
            .expect("shipped [retry] section must load");
        assert_eq!(
            cfg.max_retries, 3,
            "shipped config/application.toml must seed max_retries=3"
        );
    }

    /// @covers: with_config_dir
    #[test]
    fn test_with_config_dir_pointed_elsewhere_fails_to_find_the_section() {
        use crate::api::RetryConfig;

        // Pointing at a directory with no application.toml at all must fail
        // to find the [retry] section — proving with_config_dir genuinely
        // redirects the loader rather than being a no-op that always falls
        // back to the crate's shipped config/ directory.
        let loader = ApplicationConfigBuilder::new()
            .with_config_dir(env!("CARGO_MANIFEST_DIR"))
            .build_loader()
            .expect("build_loader itself must still succeed");
        let cfg: Result<RetryConfig, _> = loader.load_section("retry");
        assert!(
            cfg.is_err(),
            "a config dir with no application.toml must fail to load [retry]"
        );
    }
}
