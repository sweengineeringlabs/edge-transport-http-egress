//! `impl Processor for HttpRetrySvc` — the primary processor contract.

use crate::api::ApplicationConfigBuilder;
use crate::api::DecorateRequest;
use crate::api::DecorateResponse;
use crate::api::HttpRetrySvc;
use crate::api::Processor;
use crate::api::ProcessorRequest;
use crate::api::ProcessorResponse;
use crate::api::RetryConfigBuilder;
use crate::api::RetryError;
use crate::api::RetryLayer;

impl Processor for HttpRetrySvc {
    fn describe(&self, req: ProcessorRequest) -> Result<ProcessorResponse, RetryError> {
        let label = if req.verbose {
            "http-retry (idempotent-method exponential backoff)".to_string()
        } else {
            "http-retry".to_string()
        };
        Ok(ProcessorResponse { label })
    }

    fn decorate(&self, req: DecorateRequest) -> Result<DecorateResponse, RetryError> {
        req.config.validate().map_err(RetryError::InvalidConfig)?;
        Ok(DecorateResponse {
            layer: RetryLayer::new(req.config),
        })
    }

    fn new_config_builder() -> RetryConfigBuilder {
        RetryConfigBuilder::new()
    }

    fn new_app_config_builder() -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::new()
    }
}
