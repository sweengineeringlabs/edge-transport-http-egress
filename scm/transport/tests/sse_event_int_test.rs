//! Integration tests for `SseEvent`.

use edge_transport_http_egress_transport::SseEvent;

/// @covers: data
#[test]
fn test_sse_event_struct_data_sets_payload_and_leaves_optional_fields_none() {
    let ev = SseEvent::data("hello");
    assert_eq!(ev.data, "hello");
    assert!(ev.event.is_none());
    assert!(ev.id.is_none());
}

#[test]
fn test_sse_event_struct_full_fields_are_preserved() {
    let ev = SseEvent {
        event: Some("update".into()),
        data: "{}".into(),
        id: Some("42".into()),
    };
    assert_eq!(ev.event.as_deref(), Some("update"));
    assert_eq!(ev.id.as_deref(), Some("42"));
}
