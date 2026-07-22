//! Integration tests for `HttpRequestBuilder`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Duration;

use edge_transport_http_egress_transport::{HttpMethod, HttpRequestBuilder};

#[test]
fn test_http_request_builder_struct_new_creates_builder_with_method_and_url() {
    let req = HttpRequestBuilder::new(HttpMethod::Get, "https://api.example.com").build();
    assert_eq!(req.method, HttpMethod::Get);
    assert_eq!(req.url, "https://api.example.com");
}

/// @covers: with_header
#[test]
fn test_http_request_builder_struct_with_header_inserts_header() {
    let req = HttpRequestBuilder::new(HttpMethod::Post, "/api")
        .with_header("Content-Type", "application/json")
        .build();
    assert_eq!(
        req.headers.get("Content-Type").map(String::as_str),
        Some("application/json")
    );
}

/// @covers: with_query
#[test]
fn test_http_request_builder_struct_with_query_inserts_query_param() {
    let req = HttpRequestBuilder::new(HttpMethod::Get, "/search")
        .with_query("q", "rust")
        .build();
    assert_eq!(req.query.get("q").map(String::as_str), Some("rust"));
}

/// @covers: with_timeout
#[test]
fn test_http_request_builder_struct_with_timeout_sets_timeout() {
    let req = HttpRequestBuilder::new(HttpMethod::Get, "/")
        .with_timeout(Duration::from_secs(10))
        .build();
    assert_eq!(req.timeout, Some(Duration::from_secs(10)));
}

/// @covers: build
#[test]
fn test_http_request_builder_struct_build_returns_request_with_all_settings() {
    let req = HttpRequestBuilder::new(HttpMethod::Delete, "/resource")
        .with_header("Authorization", "Bearer tok")
        .with_query("force", "true")
        .build();
    assert_eq!(req.method, HttpMethod::Delete);
    assert!(req.headers.contains_key("Authorization"));
    assert!(req.query.contains_key("force"));
}
