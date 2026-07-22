//! Integration tests for `ConfigResponse`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::{ConfigResponse, HttpConfig};

/// @covers: ConfigResponse
#[test]
fn test_config_response_struct_carries_config_verbatim() {
    let config = HttpConfig::with_base_url("https://example.com");
    let response = ConfigResponse {
        config: config.clone(),
    };
    assert_eq!(response.config.base_url, config.base_url);
}
