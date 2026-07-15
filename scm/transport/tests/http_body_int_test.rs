//! Integration tests for `HttpBody`.

use edge_transport_http_egress_transport::HttpBody;

#[test]
fn test_http_body_enum_json_holds_value() {
    let body = HttpBody::Json(serde_json::json!({"k": "v"}));
    assert!(matches!(body, HttpBody::Json(_)));
}

#[test]
fn test_http_body_enum_raw_holds_bytes() {
    let body = HttpBody::Raw(vec![1, 2, 3]);
    assert!(matches!(body, HttpBody::Raw(ref b) if b == &[1, 2, 3]));
}
