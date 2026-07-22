//! Integration tests for `ProcessorRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::{HttpRetrySvc, Processor, ProcessorRequest};

/// @covers: describe
#[test]
fn test_processor_request_verbose_flag_changes_the_label_happy() {
    let terse = HttpRetrySvc
        .describe(ProcessorRequest { verbose: false })
        .expect("infallible");
    let verbose = HttpRetrySvc
        .describe(ProcessorRequest { verbose: true })
        .expect("infallible");
    assert_ne!(
        terse.label, verbose.label,
        "verbose=true must change the label, proving the field is actually read"
    );
}

/// @covers: describe
#[test]
fn test_processor_request_is_reusable_across_calls_edge() {
    let a = HttpRetrySvc
        .describe(ProcessorRequest { verbose: false })
        .expect("infallible");
    let b = HttpRetrySvc
        .describe(ProcessorRequest { verbose: false })
        .expect("infallible");
    assert_eq!(a.label, b.label);
}
