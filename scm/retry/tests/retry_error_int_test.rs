//! Integration tests for `retry_error` in `edge-transport-http-egress-retry`.

use edge_transport_http_egress_retry::RetryError;

/// @covers: RetryError
#[test]
fn test_retry_error_is_accessible() {
    // Construct the public error and assert its Display is actionable.
    let err = RetryError::ParseFailed("unknown field".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("edge_transport_http_egress_retry") && msg.contains("unknown field"),
        "RetryError Display must name the crate and echo the reason; got: {msg}"
    );
}

/// @covers: RetryError::ParseFailed
#[test]
fn test_retry_error_parse_failed_variant_formats_message() {
    let err = RetryError::ParseFailed("bad field".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("bad field"),
        "ParseFailed must include the underlying message; got: {msg}"
    );
}
