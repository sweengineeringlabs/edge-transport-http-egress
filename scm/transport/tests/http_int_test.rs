//! Integration tests covering the `http` crate dependency.
//!
//! Verifies that HTTP status codes, header names, and method values from the
//! `http` crate flow correctly through domain value objects and the SAF layer.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{HttpMethod, HttpRequest, HttpResponse};
use http::{Method, StatusCode};

/// @covers: HttpMethod
#[test]
fn test_http_method_get_round_trips_through_http_crate() {
    let req = HttpRequest::get("https://example.com/api");
    assert_eq!(req.method, HttpMethod::Get);
    // Round-trip: domain method → string → http::Method
    let method_str = req.method.to_string();
    let http_method = Method::from_bytes(method_str.as_bytes()).unwrap();
    assert_eq!(http_method, Method::GET);
}

/// @covers: HttpMethod
#[test]
fn test_http_method_post_round_trips_through_http_crate() {
    let req = HttpRequest::post("/data");
    let http_method = Method::from_bytes(req.method.to_string().as_bytes()).unwrap();
    assert_eq!(http_method, Method::POST);
}

/// @covers: HttpMethod
#[test]
fn test_http_method_put_round_trips_through_http_crate() {
    let req = HttpRequest::put("/resource");
    let http_method = Method::from_bytes(req.method.to_string().as_bytes()).unwrap();
    assert_eq!(http_method, Method::PUT);
}

/// @covers: HttpMethod
#[test]
fn test_http_method_delete_round_trips_through_http_crate() {
    let req = HttpRequest::delete("/resource");
    let http_method = Method::from_bytes(req.method.to_string().as_bytes()).unwrap();
    assert_eq!(http_method, Method::DELETE);
}

/// @covers: HttpResponse
#[test]
fn test_http_response_status_matches_http_status_code() {
    let resp = HttpResponse::new(200, vec![]);
    let status = StatusCode::from_u16(resp.status).unwrap();
    assert!(status.is_success());
    assert_eq!(status.as_u16(), 200);
}

/// @covers: HttpResponse
#[test]
fn test_http_response_404_is_client_error_in_http_crate() {
    let resp = HttpResponse::new(404, vec![]);
    let status = StatusCode::from_u16(resp.status).unwrap();
    assert!(status.is_client_error());
    assert!(!status.is_server_error());
}

/// @covers: HttpResponse
#[test]
fn test_http_response_500_is_server_error_in_http_crate() {
    let resp = HttpResponse::new(500, vec![]);
    let status = StatusCode::from_u16(resp.status).unwrap();
    assert!(status.is_server_error());
    assert!(!status.is_client_error());
}

/// @covers: HttpResponse
#[test]
fn test_http_response_json_parses_body_bytes() {
    let payload = serde_json::json!({"id": 42, "name": "test"});
    let body = serde_json::to_vec(&payload).unwrap();
    let resp = HttpResponse::new(200, body);
    let parsed: serde_json::Value = resp.json().unwrap();
    assert_eq!(parsed["id"], 42);
    assert_eq!(parsed["name"], "test");
}
