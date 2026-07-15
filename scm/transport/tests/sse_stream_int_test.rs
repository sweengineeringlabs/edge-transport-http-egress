//! Integration tests for `SseStream`.

use edge_transport_http_egress_transport::SseStream;
use futures::stream;

#[test]
fn test_sse_stream_type_empty_stream_is_valid() {
    let _s: SseStream = Box::pin(stream::empty());
}
