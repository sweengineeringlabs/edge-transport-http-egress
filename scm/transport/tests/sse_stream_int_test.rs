//! Integration tests for `SseStream`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{SseEvent, SseStream};
use futures::stream;
use futures::StreamExt as _;

/// @covers: SseStream
#[tokio::test]
async fn test_sse_stream_type_empty_stream_is_valid() {
    let mut s = SseStream::new(stream::empty());
    assert!(
        s.next().await.is_none(),
        "an empty SseStream must yield no events"
    );
}

/// @covers: SseStream
#[tokio::test]
async fn test_sse_stream_type_yields_pushed_events_in_order_edge() {
    let events = SseStream::new(stream::iter(vec![
        Ok(SseEvent::data("first")),
        Ok(SseEvent::data("second")),
    ]));
    let collected: Vec<_> = events.collect().await;
    assert_eq!(collected.len(), 2, "stream must yield both pushed events");
    assert_eq!(
        collected[0].as_ref().expect("first event is Ok").data,
        "first",
        "SseStream must preserve event order and payload"
    );
}
