//! Integration tests for the SAF `ProcessorFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{DescribeRequest, ProcessorFactory};

/// @covers: ProcessorFactory
#[test]
fn test_processor_factory_create_returns_working_processor() {
    let processor = ProcessorFactory::create();
    let resp = processor
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(
        resp.value, "http-rate",
        "factory-built Processor must identify as the rate processor"
    );
}
