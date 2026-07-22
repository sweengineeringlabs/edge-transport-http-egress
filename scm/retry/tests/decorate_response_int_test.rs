//! Integration tests for `DecorateResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::{
    DecorateRequest, DecorateResponse, Processor, ProcessorRequest, ProcessorResponse, RetryConfig,
    RetryError,
};

/// A minimal external test-double proving `Processor::decorate` can
/// genuinely fail for a real implementor — the crate's own `HttpRetrySvc`
/// only fails when the config itself is invalid, so this test-double
/// exercises a distinct failure mode (no config resolved at all).
struct UnconfiguredProcessor;

impl Processor for UnconfiguredProcessor {
    fn describe(&self, _req: ProcessorRequest) -> Result<ProcessorResponse, RetryError> {
        Ok(ProcessorResponse {
            label: "unconfigured".to_string(),
        })
    }

    fn decorate(&self, _req: DecorateRequest) -> Result<DecorateResponse, RetryError> {
        Err(RetryError::InvalidConfig(
            "no policy resolved for this processor".to_string(),
        ))
    }

    fn new_config_builder() -> edge_transport_http_egress_retry::RetryConfigBuilder {
        edge_transport_http_egress_retry::RetryConfigBuilder::new()
    }

    fn new_app_config_builder() -> edge_transport_http_egress_retry::ApplicationConfigBuilder {
        panic!("not exercised by this test-double")
    }
}

/// @covers: decorate
#[test]
fn test_decorate_response_unconfigured_implementor_returns_err_error() {
    let processor = UnconfiguredProcessor;
    let result = processor.decorate(DecorateRequest {
        config: RetryConfig::default(),
    });
    assert!(
        matches!(result, Err(RetryError::InvalidConfig(_))),
        "an external Processor impl reporting no resolved policy must surface as InvalidConfig; got: {result:?}"
    );
}
