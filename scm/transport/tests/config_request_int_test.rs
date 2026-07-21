//! Integration tests for `ConfigRequest`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::ConfigRequest;

/// @covers: ConfigRequest
#[test]
fn test_config_request_struct_is_a_stable_equatable_marker() {
    let a = ConfigRequest;
    let b = a;
    assert_eq!(a, b, "two constructions of the marker must compare equal");
    assert_eq!(format!("{a:?}"), "ConfigRequest");
}
