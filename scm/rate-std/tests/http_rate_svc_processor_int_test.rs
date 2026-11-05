//! Integration tests for the `HttpRateSvcProcessor` factory type.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate_api::{DescribeRequest, RateFactory};
use edge_transport_http_egress_rate_std::StdRateFactory;

/// @covers: HttpRateSvcProcessor
#[test]
fn test_http_rate_svc_processor_implements_processor() {
    // The factory type doubles as this crate's Processor identity.
    let resp = StdRateFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(
        resp.value, "http-rate",
        "StdRateFactory.create_processor() must identify as the rate processor"
    );
}
