//! Integration tests for `AlwaysValidConfig`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{
    AlwaysValidConfig, HttpConfig, HttpTransportSvc, ValidatableHttpConfig,
};

#[test]
fn test_always_valid_config_struct_validate_returns_ok() {
    // AlwaysValidConfig always passes; a zero-timeout config routed through the
    // same gateway fails — so the Ok verdict is a real result, not a gateway
    // that unconditionally returns Ok.
    assert!(
        HttpTransportSvc::validate(&AlwaysValidConfig).is_ok(),
        "AlwaysValidConfig must pass validation"
    );
    let bad = ValidatableHttpConfig {
        config: HttpConfig {
            timeout_secs: 0,
            ..HttpConfig::default()
        },
    };
    assert!(
        HttpTransportSvc::validate(&bad).is_err(),
        "a zero-timeout config must fail through the same gateway"
    );
}
