//! Integration tests for `HttpByteStream`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpByteStream;
use futures::stream;
use futures::StreamExt as _;

/// `HttpByteStream::new` must wrap the supplied stream faithfully — an empty
/// source stream must yield no chunks.
#[tokio::test]
async fn test_http_byte_stream_new_empty_source_is_valid_happy() {
    let mut s = HttpByteStream::new(stream::empty());
    assert!(
        s.next().await.is_none(),
        "an empty HttpByteStream must yield no chunks"
    );
}

/// Pushed chunks must be yielded in order and unchanged — proving the
/// wrapper delegates `poll_next` rather than dropping/reordering items.
#[tokio::test]
async fn test_http_byte_stream_yields_pushed_chunks_in_order_edge() {
    let s = HttpByteStream::new(stream::iter(vec![Ok(vec![1u8, 2]), Ok(vec![3u8])]));
    let chunks: Vec<_> = s.collect().await;
    assert_eq!(chunks.len(), 2, "stream must yield both pushed chunks");
    assert_eq!(
        chunks[0].as_ref().expect("first chunk is Ok"),
        &vec![1u8, 2],
        "HttpByteStream must preserve chunk order and content"
    );
}
