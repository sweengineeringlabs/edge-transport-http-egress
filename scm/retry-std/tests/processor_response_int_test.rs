//! Integration tests for `ProcessorResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry_api::{ProcessorRequest, RetryFactory};
use edge_transport_http_egress_retry_std::StdRetryFactory;

/// @covers: ProcessorResponse
#[test]
fn test_processor_response_label_identifies_the_retry_processor_happy() {
    let resp = StdRetryFactory
        .create_processor()
        .describe(ProcessorRequest { verbose: false })
        .expect("infallible");
    assert!(resp.label.contains("retry"));
}

/// @covers: ProcessorResponse
#[test]
fn test_processor_response_label_is_an_owned_string_edge() {
    let resp = StdRetryFactory
        .create_processor()
        .describe(ProcessorRequest { verbose: false })
        .expect("infallible");
    let label: String = resp.label;
    assert!(!label.is_empty());
}
