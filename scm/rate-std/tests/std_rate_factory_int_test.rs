//! Integration tests for `StdRateFactory`, the standard `RateFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::ConfigBuilder as _;
use edge_transport_http_egress_rate_api::{
    ConfigValidationRequest, DescribeRequest, RateConfig, RateError, RateFactory, RateLimitRequest,
};
use edge_transport_http_egress_rate_std::StdRateFactory;

fn config_with_rate(tokens_per_second: u32) -> RateConfig {
    RateConfig {
        tokens_per_second,
        ..RateConfig::default()
    }
}

/// @covers: create_processor
#[test]
fn test_create_processor_describes_the_http_rate_limiter_happy() {
    let resp = StdRateFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(resp.value, "http-rate");
}

/// @covers: create_validator
#[test]
fn test_create_validator_accepts_the_default_config_happy() {
    let result = StdRateFactory
        .create_validator()
        .validate(ConfigValidationRequest {
            config: RateConfig::default(),
        });
    assert!(
        matches!(result, Ok(())),
        "the default config must validate, got: {result:?}"
    );
}

/// @covers: create_validator
#[test]
fn test_create_validator_rejects_a_zero_token_rate_error() {
    let err = StdRateFactory
        .create_validator()
        .validate(ConfigValidationRequest {
            config: config_with_rate(0),
        })
        .expect_err("a limiter that refills no tokens is invalid");
    assert!(
        matches!(&err, RateError::InvalidConfig(msg) if msg.contains("tokens_per_second")),
        "the error must name the field, got: {err:?}"
    );
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_seeds_the_package_name_happy() {
    let builder = StdRateFactory.create_config_builder();
    // The name is a config search directory segment, so it is the package name.
    assert_eq!(builder.name(), "edge-transport-http-egress-rate-std");
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
}

/// @covers: build_rate_layer
#[test]
fn test_build_rate_layer_reports_the_configured_token_rate_happy() {
    let layer = StdRateFactory
        .build_rate_layer(config_with_rate(7))
        .expect("a valid config must build");
    let resp = layer
        .rate_limit(RateLimitRequest)
        .expect("rate_limit is infallible");
    assert_eq!(
        resp.tokens_per_second, 7,
        "the layer must carry the configured rate"
    );
}

/// @covers: build_rate_layer
#[test]
fn test_build_rate_layer_rejects_an_invalid_config_error() {
    let result = StdRateFactory.build_rate_layer(config_with_rate(0));
    assert!(
        matches!(result, Err(RateError::InvalidConfig(_))),
        "building must validate the config, got: {:?}",
        result.map(|_| "a layer")
    );
}

/// @covers: build_rate_layer
#[test]
fn test_build_rate_layer_called_twice_builds_independent_layers_edge() {
    let a = StdRateFactory
        .build_rate_layer(config_with_rate(2))
        .unwrap();
    let b = StdRateFactory
        .build_rate_layer(config_with_rate(9))
        .unwrap();
    assert_eq!(a.rate_limit(RateLimitRequest).unwrap().tokens_per_second, 2);
    assert_eq!(b.rate_limit(RateLimitRequest).unwrap().tokens_per_second, 9);
}
