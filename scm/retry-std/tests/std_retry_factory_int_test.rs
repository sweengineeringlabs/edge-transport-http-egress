//! Integration tests for `StdRetryFactory`, the standard `RetryFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry_api::{
    ProcessorRequest, RetryConfig, RetryError, RetryFactory, ValidationRequest,
};
use edge_transport_http_egress_retry_std::StdRetryFactory;

fn config_with_multiplier(multiplier: f64) -> RetryConfig {
    RetryConfig {
        multiplier,
        ..RetryConfig::default()
    }
}

/// @covers: create_processor
#[test]
fn test_create_processor_describes_the_retry_middleware_happy() {
    let processor = StdRetryFactory.create_processor();
    let plain = processor
        .describe(ProcessorRequest { verbose: false })
        .expect("describe is infallible");
    assert_eq!(plain.label, "http-retry");
    let verbose = processor
        .describe(ProcessorRequest { verbose: true })
        .expect("describe is infallible");
    assert!(
        verbose.label.starts_with("http-retry (") && verbose.label.contains("backoff"),
        "the verbose label must explain the policy, got: {}",
        verbose.label
    );
}

/// @covers: create_validator
#[test]
fn test_create_validator_accepts_the_default_config_happy() {
    let result = StdRetryFactory
        .create_validator()
        .validate(ValidationRequest {
            config: RetryConfig::default(),
        });
    assert!(
        matches!(result, Ok(())),
        "the default config must validate, got: {result:?}"
    );
}

/// @covers: create_validator
#[test]
fn test_create_validator_rejects_a_zero_multiplier_error() {
    let err = StdRetryFactory
        .create_validator()
        .validate(ValidationRequest {
            config: config_with_multiplier(0.0),
        })
        .expect_err("a backoff multiplier of zero is invalid");
    assert!(
        matches!(&err, RetryError::InvalidConfig(msg) if msg.contains("multiplier")),
        "the error must name the field, got: {err:?}"
    );
}

/// @covers: new_config_builder
#[test]
fn test_new_config_builder_builds_a_config_with_the_chosen_retry_budget_happy() {
    let config = StdRetryFactory
        .new_config_builder()
        .max_retries(6)
        .build()
        .expect("a builder with defaults plus a retry budget must build");
    assert_eq!(config.max_retries, 6);
}

/// @covers: new_app_config_builder
#[test]
fn test_new_app_config_builder_keeps_the_pre_split_application_name_happy() {
    let builder = StdRetryFactory.new_app_config_builder();
    // The name is a config search directory segment: it must not change with the package split.
    assert_eq!(builder.name(), "edge-transport-http-egress-retry");
}
