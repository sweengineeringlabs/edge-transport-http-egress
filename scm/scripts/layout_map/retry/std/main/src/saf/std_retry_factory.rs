//! The standard implementation of the retry construction contract.

use edge_transport_http_egress_retry_api::{
    ApplicationConfigBuilder, Processor, RetryConfigBuilder, RetryFactory, Validator,
};

use crate::core::{DefaultValidator, HttpRetrySvc};

/// Builds the standard retry middleware and its helpers.
#[derive(Debug, Clone, Copy, Default)]
pub struct StdRetryFactory;

impl RetryFactory for StdRetryFactory {
    fn create_processor(&self) -> Box<dyn Processor> {
        Box::new(HttpRetrySvc)
    }

    fn create_validator(&self) -> Box<dyn Validator> {
        Box::new(DefaultValidator)
    }

    fn new_config_builder(&self) -> RetryConfigBuilder {
        RetryConfigBuilder::new()
    }

    fn new_app_config_builder(&self) -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::new()
    }
}
