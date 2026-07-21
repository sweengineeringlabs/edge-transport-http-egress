//! Integration tests for the `Processor` trait in `edge-transport-http-egress-rate`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{DescribeRequest, HttpRateSvcProcessor, Processor};

/// @covers: Processor
#[test]
fn test_processor_describe_returns_crate_label() {
    // `HttpRateSvcProcessor` implements `Processor`; `describe()` must return the
    // processor's canonical label. Asserting the value proves the impl is
    // real, not just that the type constructs.
    let svc = HttpRateSvcProcessor;
    let resp = svc
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(
        resp.value, "http-rate",
        "Processor::describe must return the rate processor label"
    );
}
