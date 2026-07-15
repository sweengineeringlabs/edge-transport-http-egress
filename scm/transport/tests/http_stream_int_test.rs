//! Integration tests for `HttpStream`.

use edge_transport_http_egress_transport::HttpStream;

#[test]
fn test_http_stream_trait_is_object_safe() {
    fn _assert_object_safe(_: &dyn HttpStream) {}
}
