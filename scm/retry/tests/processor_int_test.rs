//! Integration tests for the `Processor` trait in `edge-transport-http-egress-retry`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::{HttpRetrySvc, Processor, ProcessorRequest};

/// @covers: Processor
#[test]
fn test_processor_trait_is_implementable() {
    // HttpRetrySvc implements Processor to satisfy the service_type contract.
    let svc = HttpRetrySvc;
    let resp = svc
        .describe(ProcessorRequest { verbose: false })
        .expect("describe is infallible");
    assert_eq!(
        resp.label, "http-retry",
        "Processor::describe must identify the retry processor unit"
    );
}
