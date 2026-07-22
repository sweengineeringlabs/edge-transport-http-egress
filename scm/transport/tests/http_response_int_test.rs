//! Integration tests for `HttpResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpResponse;

/// @covers: is_success
#[test]
fn test_http_response_struct_is_success_returns_true_for_2xx_status() {
    assert!(HttpResponse::new(200, vec![]).is_success());
    assert!(HttpResponse::new(299, vec![]).is_success());
    assert!(!HttpResponse::new(400, vec![]).is_success());
}

/// @covers: is_client_error
#[test]
fn test_http_response_struct_is_client_error_returns_true_for_4xx_status() {
    assert!(HttpResponse::new(404, vec![]).is_client_error());
    assert!(!HttpResponse::new(200, vec![]).is_client_error());
}

/// @covers: is_server_error
#[test]
fn test_http_response_struct_is_server_error_returns_true_for_5xx_status() {
    assert!(HttpResponse::new(500, vec![]).is_server_error());
    assert!(!HttpResponse::new(200, vec![]).is_server_error());
}

/// @covers: text
#[test]
fn test_http_response_struct_text_returns_utf8_string_from_body() {
    let resp = HttpResponse::new(200, b"hello".to_vec());
    assert_eq!(resp.text().unwrap(), "hello");
}

/// @covers: header
#[test]
fn test_http_response_struct_header_returns_value_for_exact_case_match() {
    let mut resp = HttpResponse::new(200, vec![]);
    resp.headers
        .insert("Content-Type".to_string(), "text/html".to_string());
    assert_eq!(resp.header("Content-Type"), Some("text/html"));
    assert!(resp.header("X-Missing").is_none());
}

/// @covers: header
#[test]
fn test_http_response_struct_header_returns_value_for_lowercase_lookup() {
    let mut resp = HttpResponse::new(200, vec![]);
    resp.headers
        .insert("Content-Type".to_string(), "text/html".to_string());
    assert_eq!(resp.header("content-type"), Some("text/html"));
}

/// @covers: header
#[test]
fn test_http_response_struct_header_returns_value_for_mixed_case_lookup() {
    let mut resp = HttpResponse::new(200, vec![]);
    resp.headers
        .insert("Content-Type".to_string(), "text/html".to_string());
    assert_eq!(resp.header("CONTENT-TYPE"), Some("text/html"));
}

/// @covers: json
#[test]
fn test_http_response_struct_json_parses_body_as_json_value() {
    let data = serde_json::json!({"name": "test"});
    let resp = HttpResponse::new(200, serde_json::to_vec(&data).unwrap());
    let parsed: serde_json::Value = resp.json().unwrap();
    assert_eq!(parsed["name"], "test");
}
