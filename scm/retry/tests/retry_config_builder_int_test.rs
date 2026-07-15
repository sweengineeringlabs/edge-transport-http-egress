//! Integration tests for `RetryConfigBuilder`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_retry::RetryConfigBuilder;

/// @covers: RetryConfigBuilder::new
#[test]
fn retry_struct_retry_config_builder_new_creates_with_defaults_int_test() {
    let cfg = RetryConfigBuilder::new()
        .build()
        .expect("defaults must be valid");
    assert!(cfg.max_retries > 0, "default max_retries must be > 0");
    assert!(cfg.multiplier > 0.0, "default multiplier must be > 0");
}

/// @covers: RetryConfigBuilder::max_retries
#[test]
fn retry_struct_retry_config_builder_max_retries_sets_field_int_test() {
    let cfg = RetryConfigBuilder::new()
        .max_retries(7)
        .build()
        .expect("must build");
    assert_eq!(cfg.max_retries, 7);
}

/// @covers: RetryConfigBuilder::initial_interval_ms
#[test]
fn retry_struct_retry_config_builder_initial_interval_ms_sets_field_int_test() {
    let cfg = RetryConfigBuilder::new()
        .initial_interval_ms(200)
        .build()
        .expect("must build");
    assert_eq!(cfg.initial_interval_ms, 200);
}

/// @covers: RetryConfigBuilder::max_interval_ms
#[test]
fn retry_struct_retry_config_builder_max_interval_ms_sets_field_int_test() {
    let cfg = RetryConfigBuilder::new()
        .max_interval_ms(5000)
        .build()
        .expect("must build");
    assert_eq!(cfg.max_interval_ms, 5000);
}

/// @covers: RetryConfigBuilder::retryable_statuses
#[test]
fn retry_struct_retry_config_builder_retryable_statuses_sets_field_int_test() {
    let cfg = RetryConfigBuilder::new()
        .retryable_statuses(vec![503, 429])
        .build()
        .expect("must build");
    assert!(cfg.retryable_statuses.contains(&503));
    assert!(cfg.retryable_statuses.contains(&429));
}

/// @covers: RetryConfigBuilder::retryable_methods
#[test]
fn retry_struct_retry_config_builder_retryable_methods_sets_field_int_test() {
    let cfg = RetryConfigBuilder::new()
        .retryable_methods(vec!["GET".to_string(), "HEAD".to_string()])
        .build()
        .expect("must build");
    assert!(cfg.retryable_methods.contains(&"GET".to_string()));
}

/// @covers: RetryConfigBuilder::build
#[test]
fn retry_struct_retry_config_builder_build_rejects_zero_multiplier_int_test() {
    let result = RetryConfigBuilder::new().multiplier(0.0).build();
    assert!(result.is_err(), "multiplier=0 must fail validation");
}

/// @covers: RetryConfigBuilder::build
#[test]
fn retry_struct_retry_config_builder_build_rejects_max_less_than_initial_int_test() {
    let result = RetryConfigBuilder::new()
        .initial_interval_ms(1000)
        .max_interval_ms(100)
        .build();
    assert!(result.is_err(), "max < initial must fail validation");
}
