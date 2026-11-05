//! `impl Processor for HttpCacheSvcProcessor` — this crate's `Processor` identity,
//! plus its factory-method surface.
//!
//! `HttpCacheSvcProcessor` is otherwise a static factory namespace (see `saf/`); this
//! impl gives it a genuine role as a trait implementor, matching the
//! `Processor` contract declared in `api/`.

use super::cache_svc_processor::HttpCacheSvcProcessor;
use crate::core::cache::MiddlewareHttpCache;
use configbuilder::ConfigBuilder;
use edge_transport_http_egress_cache_api::{
    CacheConfig, CacheError, DescribeRequest, DescribeResponse, Processor,
};

impl Processor for HttpCacheSvcProcessor {
    fn describe(&self, _request: DescribeRequest) -> Result<DescribeResponse, CacheError> {
        Ok(DescribeResponse {
            value: "http-cache".to_string(),
        })
    }
}

impl HttpCacheSvcProcessor {
    /// The application name the config builder is seeded with. configbuilder uses it as a directory
    /// segment when it searches for configuration, so it stays the name of the package this
    /// middleware had before the `-api`/`-std` split; seeding the `-std` package name would change
    /// where existing deployments' configuration is found.
    const CONFIG_APPLICATION_NAME: &'static str = "edge-transport-http-egress-cache";

    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> configbuilder::ConfigBuilderImpl {
        let mut b = configbuilder::ConfigLoaderFactory::create_config_builder();
        b = b.with_name(Self::CONFIG_APPLICATION_NAME);
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build a [`MiddlewareHttpCache`] from a caller-supplied [`CacheConfig`].
    pub fn build_cache_layer(config: CacheConfig) -> Result<MiddlewareHttpCache, CacheError> {
        let layer = MiddlewareHttpCache::new(config);
        Ok(layer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_label() {
        let resp = HttpCacheSvcProcessor
            .describe(DescribeRequest)
            .expect("describe is infallible");
        assert_eq!(resp.value, "http-cache");
    }

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_seeds_crate_name_inline() {
        let builder = HttpCacheSvcProcessor::create_config_builder();
        assert_eq!(builder.name(), "edge-transport-http-egress-cache");
    }

    /// @covers: build_cache_layer
    #[test]
    fn test_build_cache_layer_succeeds_with_default_config_inline() {
        let layer = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default())
            .expect("default must build");
        assert!(format!("{layer:?}").contains("300"));
    }
}
