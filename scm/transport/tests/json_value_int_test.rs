//! Integration tests for `JsonValue`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::JsonValue;

/// `JsonValue::new` must round-trip through `serde_json` transparently —
/// serialized output must be the raw JSON value, not a wrapper envelope.
#[test]
fn test_json_value_new_serializes_transparently_happy() {
    let value = JsonValue::new(serde_json::json!({"k": "v"}));
    let serialized = serde_json::to_string(&value).expect("must serialize");
    assert_eq!(
        serialized, r#"{"k":"v"}"#,
        "JsonValue must serialize as the raw JSON value, not a tuple/wrapper"
    );
}

/// Two `JsonValue`s built from different JSON must serialize differently —
/// proving the wrapper isn't discarding or ignoring the wrapped payload.
#[test]
fn test_json_value_new_preserves_distinct_payloads_edge() {
    let a = JsonValue::new(serde_json::json!({"k": "a"}));
    let b = JsonValue::new(serde_json::json!({"k": "b"}));
    let a_str = serde_json::to_string(&a).expect("must serialize");
    let b_str = serde_json::to_string(&b).expect("must serialize");
    assert_ne!(a_str, b_str);
}
