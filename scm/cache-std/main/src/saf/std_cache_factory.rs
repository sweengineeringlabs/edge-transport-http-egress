//! The standard implementation of the HTTP-cache construction contract.

use std::sync::Arc;

use edge_transport_http_egress_cache_api::{
    CacheConfig, CacheError, CacheFactory, CacheLayer, Processor, Validator,
};

use crate::core::cache::default_validator::DefaultValidator;
use crate::core::processor::cache_svc_processor::HttpCacheSvcProcessor;

/// Builds the standard HTTP cache and its helpers.
#[derive(Debug, Clone, Copy, Default)]
pub struct StdCacheFactory;

impl CacheFactory for StdCacheFactory {
    fn create_processor(&self) -> Box<dyn Processor> {
        Box::new(HttpCacheSvcProcessor)
    }

    fn create_validator(&self) -> Box<dyn Validator> {
        Box::new(DefaultValidator)
    }

    fn create_config_builder(&self) -> configbuilder::ConfigBuilderImpl {
        HttpCacheSvcProcessor::create_config_builder()
    }

    fn build_cache_layer(&self, config: CacheConfig) -> Result<Arc<dyn CacheLayer>, CacheError> {
        let layer = HttpCacheSvcProcessor::build_cache_layer(config)?;
        Ok(Arc::new(layer))
    }
}
