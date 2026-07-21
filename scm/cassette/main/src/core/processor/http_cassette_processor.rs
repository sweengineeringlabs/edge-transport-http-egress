//! `impl Processor for HttpCassetteSvc` — this crate's `Processor` identity,
//! plus its factory-method surface.
//!
//! `HttpCassetteSvc` is otherwise a static factory namespace (see `saf/`);
//! this impl gives it a genuine role as a trait implementor, matching the
//! `Processor` contract declared in `api/`.

use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::api::{
    CassetteConfig, CassetteError, CassetteLayer, DescribeRequest, DescribeResponse,
    HttpCassetteSvc, Processor,
};

impl Processor for HttpCassetteSvc {
    fn describe(&self, _request: DescribeRequest) -> Result<DescribeResponse, CassetteError> {
        Ok(DescribeResponse {
            value: "http-cassette".to_string(),
        })
    }
}

impl HttpCassetteSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build a [`CassetteLayer`] from a caller-supplied config and cassette name.
    ///
    /// Resolves the cassette path (`<cassette_dir>/<cassette_name>.yaml`),
    /// loads any pre-recorded fixtures from disk, and returns a ready layer.
    pub fn build_cassette_layer(
        config: CassetteConfig,
        cassette_name: &str,
    ) -> Result<CassetteLayer, CassetteError> {
        let path = PathBuf::from(&config.cassette_dir).join(format!("{cassette_name}.yaml"));
        let fixtures = CassetteLayer::load_fixtures_from_disk(&path)?;
        Ok(CassetteLayer {
            config: Arc::new(config),
            cassette_path: path,
            fixtures: Arc::new(crate::core::cassette::fixture_store::FixtureStore(
                Mutex::new(fixtures),
            )),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_label() {
        let resp = HttpCassetteSvc
            .describe(DescribeRequest)
            .expect("describe is infallible");
        assert_eq!(resp.value, "http-cassette");
    }

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_seeds_crate_name_inline() {
        let builder = HttpCassetteSvc::create_config_builder();
        assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
    }

    /// @covers: build_cassette_layer
    #[test]
    fn test_build_cassette_layer_succeeds_with_default_config_inline() {
        let dir = tempfile::tempdir().expect("tempdir");
        let config = CassetteConfig {
            cassette_dir: dir.path().to_string_lossy().to_string(),
            ..CassetteConfig::swe_default().expect("baseline parses")
        };
        HttpCassetteSvc::build_cassette_layer(config, "inline_test")
            .expect("default config must build");
    }
}
