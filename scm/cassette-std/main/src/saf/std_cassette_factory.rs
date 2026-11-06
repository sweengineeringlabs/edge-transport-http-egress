//! The standard implementation of the cassette construction contract.

use std::sync::Arc;

use edge_transport_http_egress_cassette_api::{
    CassetteConfig, CassetteError, CassetteFactory, CassetteLayer, Processor, Validator,
};

use crate::core::cassette::default_validator::DefaultValidator;
use crate::core::processor::cassette_svc_processor::HttpCassetteSvc;

/// Builds the standard record/replay layer and its helpers.
#[derive(Debug, Clone, Copy, Default)]
pub struct StdCassetteFactory;

impl CassetteFactory for StdCassetteFactory {
    fn create_processor(&self) -> Box<dyn Processor> {
        Box::new(HttpCassetteSvc)
    }

    fn create_validator(&self) -> Box<dyn Validator> {
        Box::new(DefaultValidator)
    }

    fn create_config_builder(&self) -> configbuilder::ConfigBuilderImpl {
        HttpCassetteSvc::create_config_builder()
    }

    fn build_cassette_layer(
        &self,
        config: CassetteConfig,
        cassette_name: &str,
    ) -> Result<Arc<dyn CassetteLayer>, CassetteError> {
        let layer = HttpCassetteSvc::build_cassette_layer(config, cassette_name)?;
        Ok(Arc::new(layer))
    }
}
