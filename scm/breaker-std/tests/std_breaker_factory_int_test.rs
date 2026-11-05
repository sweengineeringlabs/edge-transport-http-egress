//! Integration tests for `StdBreakerFactory`, the standard `BreakerFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use configbuilder::ConfigBuilder as _;
use edge_transport_http_egress_breaker_api::{
    BreakerConfig, BreakerError, BreakerFactory, ConfigValidationRequest, DescribeRequest,
    FailureThresholdRequest,
};
use edge_transport_http_egress_breaker_std::StdBreakerFactory;

fn config_with_threshold(failure_threshold: u32) -> BreakerConfig {
    BreakerConfig {
        failure_threshold,
        ..BreakerConfig::default()
    }
}

/// @covers: create_processor
#[test]
fn test_create_processor_describes_the_http_breaker_happy() {
    let resp = StdBreakerFactory
        .create_processor()
        .describe(DescribeRequest)
        .expect("describe is infallible");
    assert_eq!(resp.value, "http-breaker");
}

/// @covers: create_validator
#[test]
fn test_create_validator_accepts_the_default_config_happy() {
    let result = StdBreakerFactory
        .create_validator()
        .validate(ConfigValidationRequest {
            config: BreakerConfig::default(),
        });
    assert!(
        matches!(result, Ok(())),
        "the default config must validate: {result:?}"
    );
}

/// @covers: create_validator
#[test]
fn test_create_validator_rejects_a_zero_failure_threshold_error() {
    let err = StdBreakerFactory
        .create_validator()
        .validate(ConfigValidationRequest {
            config: config_with_threshold(0),
        })
        .expect_err("a breaker that trips after zero failures is invalid");
    assert!(
        matches!(&err, BreakerError::InvalidConfig(msg) if msg.contains("failure_threshold")),
        "the error must name the field, got: {err:?}"
    );
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_seeds_this_package_name_and_version_happy() {
    let builder = StdBreakerFactory.create_config_builder();
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
}

/// @covers: build_breaker_layer
#[test]
fn test_build_breaker_layer_reports_the_configured_failure_threshold_happy() {
    let layer = StdBreakerFactory
        .build_breaker_layer(config_with_threshold(7))
        .expect("a valid config must build");
    let resp = layer
        .failure_threshold(FailureThresholdRequest)
        .expect("failure_threshold is infallible");
    assert_eq!(
        resp.value, 7,
        "the layer must carry the configured threshold"
    );
}

/// @covers: build_breaker_layer
#[test]
fn test_build_breaker_layer_called_twice_builds_independent_layers_edge() {
    let a = StdBreakerFactory
        .build_breaker_layer(config_with_threshold(2))
        .expect("build a");
    let b = StdBreakerFactory
        .build_breaker_layer(config_with_threshold(9))
        .expect("build b");
    assert_eq!(
        a.failure_threshold(FailureThresholdRequest).unwrap().value,
        2
    );
    assert_eq!(
        b.failure_threshold(FailureThresholdRequest).unwrap().value,
        9
    );
}
