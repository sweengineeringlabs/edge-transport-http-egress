//! `impl Processor for HttpRetrySvc` — the primary processor contract.

use super::retry_layer::PolicyRetryLayer;
use super::svc_processor::HttpRetrySvc;
use edge_transport_http_egress_retry_api::ApplicationConfigBuilder;
use edge_transport_http_egress_retry_api::DecorateRequest;
use edge_transport_http_egress_retry_api::DecorateResponse;
use edge_transport_http_egress_retry_api::Processor;
use edge_transport_http_egress_retry_api::ProcessorRequest;
use edge_transport_http_egress_retry_api::ProcessorResponse;
use edge_transport_http_egress_retry_api::RetryConfigBuilder;
use edge_transport_http_egress_retry_api::RetryError;

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
            layer: std::sync::Arc::new(PolicyRetryLayer::new(req.config)),
        })
    }

    fn new_config_builder() -> RetryConfigBuilder {
        RetryConfigBuilder::new()
    }

    fn new_app_config_builder() -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::new()
    }
}
