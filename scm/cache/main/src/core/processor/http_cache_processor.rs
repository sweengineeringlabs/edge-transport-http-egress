//! `impl Processor for HttpCacheSvcProcessor` — this crate's `Processor` identity,
//! plus its factory-method surface.
//!
//! `HttpCacheSvcProcessor` is otherwise a static factory namespace (see `saf/`); this
//! impl gives it a genuine role as a trait implementor, matching the
//! `Processor` contract declared in `api/`.

use crate::api::{
    CacheConfig, CacheError, DescribeRequest, DescribeResponse, HttpCacheSvcProcessor,
    MiddlewareHttpCache, Processor,
};

impl Processor for HttpCacheSvcProcessor {
    fn describe(&self, _request: DescribeRequest) -> Result<DescribeResponse, CacheError> {
        Ok(DescribeResponse {
            value: "http-cache".to_string(),
        })
    }
}

impl HttpCacheSvcProcessor {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
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
        assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
    }

    /// @covers: build_cache_layer
    #[test]
    fn test_build_cache_layer_succeeds_with_default_config_inline() {
        let layer = HttpCacheSvcProcessor::build_cache_layer(CacheConfig::default())
            .expect("default must build");
        assert!(format!("{layer:?}").contains("300"));
    }
}
