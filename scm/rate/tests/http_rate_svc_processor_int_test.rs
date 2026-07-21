//! Integration tests for the `HttpRateSvcProcessor` factory type.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{DescribeRequest, HttpRateSvcProcessor, Processor};

/// @covers: HttpRateSvcProcessor
#[test]
fn test_http_rate_svc_processor_implements_processor() {
    // The factory type doubles as this crate's Processor identity.
    let resp = HttpRateSvcProcessor
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(
        resp.value, "http-rate",
        "HttpRateSvcProcessor must identify as the rate processor"
    );
}
