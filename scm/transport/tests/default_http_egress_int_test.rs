//! Integration tests for the DefaultHttpEgress SAF factories.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{HttpConfig, HttpTransportSvc};

/// @covers: default_http_egress
#[test]
fn test_default_http_egress_builds_successfully_with_swe_defaults() {
    // The returned egress is an opaque `Box<dyn HttpEgress>`; assert the factory
    // is repeatably callable (a builder with broken shared state would fail the
    // second call). End-to-end send behaviour is covered in coverage_int_test.rs.
    let first = HttpTransportSvc::default_http_egress();
    let second = HttpTransportSvc::default_http_egress();
    assert!(
        first.is_ok() && second.is_ok(),
        "default_http_egress must build repeatably with SWE defaults: {:?} / {:?}",
        first.err(),
        second.err(),
    );
}

/// @covers: default_http_egress_with_config
#[test]
fn test_default_http_egress_with_config_builds_with_custom_base_url() {
    let with_url = HttpTransportSvc::default_http_egress_with_config(HttpConfig {
        base_url: Some("http://example.com".to_string()),
        ..Default::default()
    });
    let without_url = HttpTransportSvc::default_http_egress_with_config(HttpConfig::default());
    assert!(
        with_url.is_ok() && without_url.is_ok(),
        "default_http_egress_with_config must build with and without a base_url: {:?} / {:?}",
        with_url.err(),
        without_url.err(),
    );
}

/// @covers: default_http_egress
#[test]
fn test_default_http_egress_builds_two_instances_independently() {
    let first = HttpTransportSvc::default_http_egress();
    let second = HttpTransportSvc::default_http_egress();
    assert!(
        first.is_ok() && second.is_ok(),
        "two independent default_http_egress builds must both succeed"
    );
}
