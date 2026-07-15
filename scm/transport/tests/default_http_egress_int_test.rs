//! Integration tests for the DefaultHttpEgress SAF factories.

use edge_transport_http_egress_transport::{HttpConfig, HttpTransportSvc};

/// @covers: default_http_egress
#[test]
fn test_default_http_egress_builds_successfully_with_swe_defaults() {
    let result = HttpTransportSvc::default_http_egress();
    assert!(
        result.is_ok(),
        "default_http_egress must build with SWE defaults: {:?}",
        result.err(),
    );
}

/// @covers: default_http_egress_with_config
#[test]
fn test_default_http_egress_with_config_builds_with_custom_base_url() {
    let cfg = HttpConfig {
        base_url: Some("http://example.com".to_string()),
        ..Default::default()
    };
    let result = HttpTransportSvc::default_http_egress_with_config(cfg);
    assert!(
        result.is_ok(),
        "default_http_egress_with_config must build: {:?}",
        result.err(),
    );
}

/// @covers: default_http_egress
#[test]
fn test_default_http_egress_builds_two_instances_independently() {
    let first = HttpTransportSvc::default_http_egress();
    let second = HttpTransportSvc::default_http_egress();
    assert!(first.is_ok(), "first build must succeed");
    assert!(second.is_ok(), "second build must succeed");
}
