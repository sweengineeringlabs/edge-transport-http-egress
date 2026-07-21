//! Integration tests for [`ProcessorFactory`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::{ProcessorFactory, ProcessorRequest};

/// @covers: create
#[test]
fn test_create_produces_a_working_processor_happy() {
    let processor = ProcessorFactory::create();
    let resp = processor
        .describe(ProcessorRequest { verbose: false })
        .expect("infallible");
    assert!(!resp.label.is_empty());
}

/// @covers: create
#[test]
fn test_create_identifies_as_retry_processor_error() {
    let processor = ProcessorFactory::create();
    let resp = processor
        .describe(ProcessorRequest { verbose: false })
        .expect("must succeed");
    assert!(resp.label.contains("retry"));
}

/// @covers: create
#[test]
fn test_create_produces_independent_instances_edge() {
    let first = ProcessorFactory::create();
    let second = ProcessorFactory::create();
    let resp1 = first
        .describe(ProcessorRequest { verbose: false })
        .expect("first must succeed");
    let resp2 = second
        .describe(ProcessorRequest { verbose: false })
        .expect("second must succeed");
    assert_eq!(resp1.label, resp2.label);
}
