//! Integration tests for `ValidatableHttpConfig`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{HttpConfig, HttpTransportSvc, ValidatableHttpConfig};

#[test]
fn test_validatable_http_config_struct_ok_for_defaults() {
    let v = ValidatableHttpConfig {
        config: HttpConfig::default(),
    };
    assert!(HttpTransportSvc::validate(&v).is_ok());
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
