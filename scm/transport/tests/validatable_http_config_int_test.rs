//! Integration tests for `ValidatableHttpConfig`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{HttpConfig, HttpTransportSvc, ValidatableHttpConfig};

#[test]
fn test_validatable_http_config_struct_ok_for_defaults() {
    let v = ValidatableHttpConfig {
        config: HttpConfig::default(),
    };
    assert!(
        HttpTransportSvc::validate(&v).is_ok(),
        "a default HttpConfig must be validatable"
    );
    // Sibling negative: a zero connect-timeout config must be rejected, so the
    // Ok above is a real verdict rather than an always-pass validator.
    let bad = ValidatableHttpConfig {
        config: HttpConfig {
            connect_timeout_secs: 0,
            ..HttpConfig::default()
        },
    };
    assert!(
        HttpTransportSvc::validate(&bad).is_err(),
        "a zero connect_timeout config must fail validation"
    );
}

#[test]
fn test_validatable_http_config_struct_err_for_zero_timeout() {
    let v = ValidatableHttpConfig {
        config: HttpConfig {
            timeout_secs: 0,
            ..HttpConfig::default()
        },
    };
    let err = HttpTransportSvc::validate(&v).unwrap_err();
    assert!(err.contains("timeout_secs"), "got: {err:?}");
}
