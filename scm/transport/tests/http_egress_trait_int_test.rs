//! Integration tests for `HttpEgress`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpEgress;

#[test]
fn test_http_egress_trait_is_object_safe() {
    fn _assert_object_safe(_: &dyn HttpEgress) {}
}
