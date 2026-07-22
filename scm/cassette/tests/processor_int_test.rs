//! Integration tests for the `Processor` trait in `edge-transport-http-egress-cassette`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_cassette::{
    CassetteError, DescribeRequest, DescribeResponse, HttpCassetteSvc, Processor,
};

/// @covers: Processor
/// `HttpCassetteSvc` is the crate's own `Processor` implementor — verify its
/// real, production `describe()` dispatch.
#[test]
fn test_http_cassette_svc_describe_returns_crate_label() {
    let resp = HttpCassetteSvc
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(resp.value, "http-cassette");
}

/// @covers: Processor
/// Proves `Processor` is genuinely usable by external consumers — the actual
/// contract of exporting a public trait — by implementing it locally.
struct TestProcessor;

impl Processor for TestProcessor {
    fn describe(&self, _request: DescribeRequest) -> Result<DescribeResponse, CassetteError> {
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
