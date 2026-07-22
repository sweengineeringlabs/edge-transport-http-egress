//! Integration tests for the `ConfigValidationRequest` DTO.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_rate::{ConfigValidationRequest, RateConfig};

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_wraps_config() {
    let req = ConfigValidationRequest {
        config: RateConfig {
            tokens_per_second: 42,
            burst_capacity: 84,
            per_host: false,
        },
    };
    assert_eq!(
        req.config.tokens_per_second, 42,
        "request must carry the config it wraps"
    );
    assert_eq!(req.config.burst_capacity, 84);
    assert!(!req.config.per_host);
}
