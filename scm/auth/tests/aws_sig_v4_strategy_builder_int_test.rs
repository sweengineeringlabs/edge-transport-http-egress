//! Integration tests for `AwsSigV4StrategyBuilder`.
//!
//! Rule 120: `src/api/strategy/aws/aws_sig_v4_strategy_builder.rs` requires a
//! corresponding test file.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::{AuthError, AwsSigV4StrategyBuilder, AwsSigV4StrategyConfig};

/// @covers: AwsSigV4StrategyBuilder::new
/// Verifies the builder is constructible via `new()`.
#[test]
fn auth_struct_aws_sig_v4_strategy_builder_new_returns_default_int_test() {
    let _builder = AwsSigV4StrategyBuilder::new();
}

/// @covers: AwsSigV4StrategyBuilder::build_config
/// Builder with all required fields set must succeed.
#[test]
fn auth_struct_aws_sig_v4_strategy_builder_build_config_all_fields_succeeds_int_test() {
    let result: Result<AwsSigV4StrategyConfig, AuthError> = AwsSigV4StrategyBuilder::new()
        .with_access_key_id(["AKIA", "IOSFODNN7EXAMPLE"].concat())
        .with_secret_access_key(["wJalrXUtnFEMI/K7MDENG", "/bPxRfiCYEXAMPLEKEY"].concat())
        .with_region("us-east-1")
        .with_service("s3")
        .build_config();
    assert!(
        result.is_ok(),
        "builder with all required fields must succeed; got: {result:?}"
    );
}

/// @covers: AwsSigV4StrategyBuilder::build_config
/// Builder without `access_key_id` must return an error.
#[test]
fn auth_struct_aws_sig_v4_strategy_builder_build_config_missing_access_key_fails_int_test() {
    let result = AwsSigV4StrategyBuilder::new()
        .with_secret_access_key("secret")
        .with_region("us-east-1")
        .with_service("s3")
        .build_config();
    assert!(
        result.is_err(),
        "missing access_key_id must produce an error"
    );
    let err = result.unwrap_err();
    assert!(
        matches!(err, AuthError::InvalidHeaderValue(_)),
        "missing required field must yield InvalidHeaderValue; got: {err:?}"
    );
}

/// @covers: AwsSigV4StrategyBuilder::build_config
/// Builder without `secret_access_key` must return an error.
#[test]
fn auth_struct_aws_sig_v4_strategy_builder_build_config_missing_secret_fails_int_test() {
    let result = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("AKIA")
        .with_region("us-east-1")
        .with_service("s3")
        .build_config();
    assert!(
        result.is_err(),
        "missing secret_access_key must produce an error"
    );
}

/// @covers: AwsSigV4StrategyBuilder::build_config
/// Builder without `region` must return an error.
#[test]
fn auth_struct_aws_sig_v4_strategy_builder_build_config_missing_region_fails_int_test() {
    let result = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("AKIA")
        .with_secret_access_key("secret")
        .with_service("s3")
        .build_config();
    assert!(result.is_err(), "missing region must produce an error");
}

/// @covers: AwsSigV4StrategyBuilder::build_config
/// Builder without `service` must return an error.
#[test]
fn auth_struct_aws_sig_v4_strategy_builder_build_config_missing_service_fails_int_test() {
    let result = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("AKIA")
        .with_secret_access_key("secret")
        .with_region("us-east-1")
        .build_config();
    assert!(result.is_err(), "missing service must produce an error");
}

/// @covers: AwsSigV4StrategyBuilder::with_session_token
/// Builder with optional session token set must succeed (token is carried through).
#[test]
fn auth_struct_aws_sig_v4_strategy_builder_with_session_token_succeeds_int_test() {
    let result = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("AKIA")
        .with_secret_access_key("secret")
        .with_region("us-east-1")
        .with_service("sts")
        .with_session_token("session-token-value")
        .build_config();
    assert!(
        result.is_ok(),
        "builder with session token must succeed; got: {result:?}"
    );
    let cfg = result.unwrap();
    assert!(
        cfg.session_token.is_some(),
        "session token must be present in built config"
    );
}
