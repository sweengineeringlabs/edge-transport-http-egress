//! Integration tests for `HttpStreamResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use edge_transport_http_egress_transport::{HttpByteStream, HttpStreamResponse};
use futures::stream;

#[test]
fn test_http_stream_response_struct_debug_does_not_expose_stream_internals() {
    let resp = HttpStreamResponse {
        status: 200,
        headers: HashMap::new(),
        body: HttpByteStream::new(stream::empty()),
    };
    let dbg = format!("{:?}", resp);
    assert!(dbg.contains("200"));
    assert!(dbg.contains("<stream>"));
}
