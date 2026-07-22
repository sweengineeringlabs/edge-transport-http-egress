//! Integration tests for `HttpConfig`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpConfig;

/// @covers: with_base_url
#[test]
fn test_http_config_struct_with_base_url_sets_base_url() {
    let cfg = HttpConfig::with_base_url("http://x.com");
    assert_eq!(cfg.base_url, Some("http://x.com".to_string()));
}

/// @covers: with_header
#[test]
fn test_http_config_struct_with_header_inserts_default_header() {
    let cfg = HttpConfig::default().with_header("X-Key", "val");
    assert_eq!(cfg.default_headers.get("X-Key"), Some(&"val".to_string()));
}

/// @covers: with_timeout
#[test]
fn test_http_config_struct_with_timeout_sets_timeout_secs() {
    let cfg = HttpConfig::default().with_timeout(60);
    assert_eq!(cfg.timeout_secs, 60);
}

/// @covers: default_max_response_bytes
#[test]
fn test_http_config_struct_default_max_response_bytes_is_ten_mib() {
    assert_eq!(
        HttpConfig::default_max_response_bytes(),
        Some(10 * 1024 * 1024)
    );
}
