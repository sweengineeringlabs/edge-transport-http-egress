//! Integration tests for the `Processor` trait in `edge-transport-http-egress-retry-std`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry_api::{ProcessorRequest, RetryFactory};
use edge_transport_http_egress_retry_std::StdRetryFactory;

/// @covers: Processor
#[test]
fn test_processor_trait_is_implementable() {
    // HttpRetrySvc implements Processor to satisfy the service_type contract.
    let svc = StdRetryFactory.create_processor();
    let resp = svc
        .describe(ProcessorRequest { verbose: false })
        .expect("describe is infallible");
    assert_eq!(
        resp.label, "http-retry",
        "Processor::describe must identify the retry processor unit"
    );
}
