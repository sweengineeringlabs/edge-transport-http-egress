//! Integration tests for `HttpConfigBuilder`.

use edge_transport_http_egress_transport::HttpConfigBuilder;

#[test]
fn test_http_config_builder_struct_new_creates_builder_with_default_values() {
    let cfg = HttpConfigBuilder::new().build();
    assert_eq!(cfg.timeout_secs, 30, "default timeout must be 30s");
}

/// @covers: with_base_url
#[test]
fn test_http_config_builder_struct_with_base_url_sets_base_url() {
    let cfg = HttpConfigBuilder::new()
        .with_base_url("https://api.example.com")
        .build();
    assert_eq!(cfg.base_url.as_deref(), Some("https://api.example.com"));
}

/// @covers: with_timeout_secs
#[test]
fn test_http_config_builder_struct_with_timeout_secs_overrides_default() {
    let cfg = HttpConfigBuilder::new().with_timeout_secs(60).build();
    assert_eq!(cfg.timeout_secs, 60);
}

/// @covers: with_header
#[test]
fn test_http_config_builder_struct_with_header_adds_default_header() {
    let cfg = HttpConfigBuilder::new()
        .with_header("X-Api-Key", "secret")
        .build();
    assert_eq!(
        cfg.default_headers.get("X-Api-Key").map(String::as_str),
        Some("secret")
    );
}

/// @covers: with_connect_timeout_secs
#[test]
fn test_http_config_builder_struct_with_connect_timeout_secs_overrides_default() {
    let cfg = HttpConfigBuilder::new()
        .with_connect_timeout_secs(20)
        .build();
    assert_eq!(cfg.connect_timeout_secs, 20);
}

/// @covers: with_user_agent
#[test]
fn test_http_config_builder_struct_with_user_agent_sets_user_agent_string() {
    let cfg = HttpConfigBuilder::new()
        .with_user_agent("my-client/1.0")
        .build();
    assert_eq!(cfg.user_agent.as_deref(), Some("my-client/1.0"));
}

/// @covers: build
#[test]
fn test_http_config_builder_struct_build_returns_config_with_chained_settings() {
    let cfg = HttpConfigBuilder::new()
        .with_base_url("https://svc.example.com")
        .with_timeout_secs(45)
        .with_user_agent("swe-test/1.0")
        .build();
    assert_eq!(cfg.base_url.as_deref(), Some("https://svc.example.com"));
    assert_eq!(cfg.timeout_secs, 45);
    assert_eq!(cfg.user_agent.as_deref(), Some("swe-test/1.0"));
}
