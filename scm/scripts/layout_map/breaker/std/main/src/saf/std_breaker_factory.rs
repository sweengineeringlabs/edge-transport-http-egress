//! The standard implementation of the breaker construction contract.

use std::sync::Arc;

use edge_transport_http_egress_breaker_api::{
    BreakerConfig, BreakerError, BreakerFactory, BreakerLayer, Processor, Validator,
};

use crate::core::breaker::default_validator::DefaultValidator;
use crate::core::default::http_breaker_svc_processor::HttpBreakerSvcProcessor;

/// Builds the standard per-host circuit breaker and its helpers.
#[derive(Debug, Clone, Copy, Default)]
pub struct StdBreakerFactory;

impl BreakerFactory for StdBreakerFactory {
    fn create_processor(&self) -> Box<dyn Processor> {
        Box::new(HttpBreakerSvcProcessor)
    }

    fn create_validator(&self) -> Box<dyn Validator> {
        Box::new(DefaultValidator)
    }

    fn create_config_builder(&self) -> configbuilder::ConfigBuilderImpl {
        HttpBreakerSvcProcessor::create_config_builder()
    }

    fn build_breaker_layer(
        &self,
        config: BreakerConfig,
    ) -> Result<Arc<dyn BreakerLayer>, BreakerError> {
        let layer = HttpBreakerSvcProcessor::build_breaker_layer(config)?;
        Ok(Arc::new(layer))
    }
}
