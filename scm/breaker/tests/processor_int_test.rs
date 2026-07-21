//! Integration tests for the `Processor` trait in `edge-transport-http-egress-breaker`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_breaker::{
    BreakerError, DescribeRequest, DescribeResponse, HttpBreakerSvcProcessor, Processor,
};

/// @covers: Processor
/// `HttpBreakerSvcProcessor` is the crate's own `Processor` implementor — verify its
/// real, production `describe()` dispatch.
#[test]
fn test_http_breaker_svc_describe_returns_crate_label() {
    let resp = HttpBreakerSvcProcessor
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(resp.value, "http-breaker");
}

/// @covers: Processor
/// Proves `Processor` is genuinely usable by external consumers — the actual
/// contract of exporting a public trait — by implementing it locally.
struct TestProcessor;

impl Processor for TestProcessor {
    fn describe(&self, _request: DescribeRequest) -> Result<DescribeResponse, BreakerError> {
        Ok(DescribeResponse {
            value: "test-processor".to_string(),
        })
    }
}

#[test]
fn test_processor_trait_is_implementable() {
    let p = TestProcessor;
    let resp = p.describe(DescribeRequest).expect("infallible");
    assert_eq!(
        resp.value, "test-processor",
        "an external Processor impl must dispatch to its own describe()"
    );
}
