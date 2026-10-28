//! Contract tests for the public resilient-composition error taxonomy.

use edge_transport_http_egress_resilient_port::ResilientError;

/// @covers: ResilientError
#[test]
fn port_error_display_preserves_layer_context_int_test() {
    let errors = [
        (
            ResilientError::Config("bad section".into()),
            "config error: bad section",
        ),
        (
            ResilientError::Retry("bad policy".into()),
            "retry layer error: bad policy",
        ),
        (
            ResilientError::Rate("bad policy".into()),
            "rate layer error: bad policy",
        ),
        (
            ResilientError::Breaker("bad policy".into()),
            "breaker layer error: bad policy",
        ),
        (
            ResilientError::Cache("bad policy".into()),
            "cache layer error: bad policy",
        ),
        (
            ResilientError::Cassette("bad policy".into()),
            "cassette layer error: bad policy",
        ),
    ];

    for (error, expected) in errors {
        assert_eq!(error.to_string(), expected);
    }
}
