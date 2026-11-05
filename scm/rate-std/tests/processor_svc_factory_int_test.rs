//! Integration tests for the SAF `ProcessorFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate_api::{DescribeRequest, RateFactory};
use edge_transport_http_egress_rate_std::StdRateFactory;

/// @covers: ProcessorFactory
#[test]
fn test_processor_factory_create_returns_working_processor() {
    let processor = StdRateFactory.create_processor();
    let resp = processor
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(
        resp.value, "http-rate",
        "factory-built Processor must identify as the rate processor"
    );
}
