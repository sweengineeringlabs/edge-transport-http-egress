//! The standard implementation of the rate-limit construction contract.

use std::sync::Arc;

use edge_transport_http_egress_rate_api::{
    Processor, RateConfig, RateError, RateFactory, RateLayer, Validator,
};

use crate::core::default::http_rate_svc_processor::HttpRateSvcProcessor;
use crate::core::rate::default_validator::DefaultValidator;

/// Builds the standard per-host rate limiter and its helpers.
#[derive(Debug, Clone, Copy, Default)]
pub struct StdRateFactory;

impl RateFactory for StdRateFactory {
    fn create_processor(&self) -> Box<dyn Processor> {
        Box::new(HttpRateSvcProcessor)
    }

    fn create_validator(&self) -> Box<dyn Validator> {
        Box::new(DefaultValidator)
    }

    fn create_config_builder(&self) -> configbuilder::ConfigBuilderImpl {
        HttpRateSvcProcessor::create_config_builder()
    }

    fn build_rate_layer(&self, config: RateConfig) -> Result<Arc<dyn RateLayer>, RateError> {
        let layer = HttpRateSvcProcessor::build_rate_layer(config)?;
        Ok(Arc::new(layer))
    }
}
