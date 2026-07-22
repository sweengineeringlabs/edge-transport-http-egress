#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for the `Processor` contract.

use edge_transport_http_egress_loadbalancer::{
    DescribeRequest, DescribeResponse, LoadbalancerMiddlewareError, Processor, ProcessorFactory,
};

/// @covers: describe
/// The crate's own `Processor` implementor (via the SAF factory) must report
/// this crate's canonical name.
#[test]
fn test_describe_returns_crate_name_happy() {
    let processor = ProcessorFactory::create();
    let resp = processor
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(resp.value, "edge-transport-http-egress-loadbalancer");
}

/// @covers: describe
/// Proves `Processor` is genuinely implementable by external consumers — the
/// actual contract of exporting a public trait — by implementing it locally.
struct TestProcessor;

impl Processor for TestProcessor {
    fn describe(
        &self,
        _request: DescribeRequest,
    ) -> Result<DescribeResponse, LoadbalancerMiddlewareError> {
        Ok(DescribeResponse {
            value: "test-processor".to_string(),
        })
    }
}

/// @covers: describe
#[test]
fn test_describe_external_implementor_dispatches_to_its_own_impl_edge() {
    let p = TestProcessor;
    let resp = p.describe(DescribeRequest).expect("infallible");
    assert_eq!(
        resp.value, "test-processor",
        "an external Processor impl must dispatch to its own describe()"
    );
}

/// A minimal external test-double proving `Processor::describe` can
/// genuinely fail for a real implementor — the crate's own
/// `LoadbalancerSvcProcessor` never returns `Err` here, so this is the only
/// way to exercise the error path.
struct FailingProcessor;

impl Processor for FailingProcessor {
    fn describe(
        &self,
        _request: DescribeRequest,
    ) -> Result<DescribeResponse, LoadbalancerMiddlewareError> {
        Err(LoadbalancerMiddlewareError::InvalidConfig(
            "no processor identity configured".to_string(),
        ))
    }
}

/// @covers: describe
#[test]
fn test_describe_unconfigured_implementor_returns_err_error() {
    let p = FailingProcessor;
    let result = p.describe(DescribeRequest);
    assert!(
        matches!(result, Err(LoadbalancerMiddlewareError::InvalidConfig(_))),
        "an external Processor impl reporting no identity must surface as InvalidConfig; got: {result:?}"
    );
}
