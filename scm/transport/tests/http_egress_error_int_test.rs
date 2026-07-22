//! Integration tests for `HttpEgressError`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpEgressError;

#[test]
fn test_http_egress_error_enum_connection_failed_formats_message() {
    let e = HttpEgressError::ConnectionFailed("refused".into());
    assert!(e.to_string().contains("refused"));
}

#[test]
fn test_http_egress_error_enum_timeout_formats_message() {
    let e = HttpEgressError::Timeout("deadline".into());
    assert!(e.to_string().contains("deadline"));
}
