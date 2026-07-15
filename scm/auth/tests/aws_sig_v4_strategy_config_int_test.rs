//! Integration tests for `AwsSigV4StrategyConfig`.
//!
//! Rule 120: `src/api/strategy/aws/aws_sig_v4_strategy_config.rs` requires a
//! corresponding test file.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::{AwsSigV4StrategyBuilder, AwsSigV4StrategyConfig};

/// @covers: AwsSigV4StrategyConfig Debug
/// Debug output must redact `access_key_id` and `secret_access_key` to prevent
/// credential leakage in logs.
#[test]
fn auth_struct_aws_sig_v4_strategy_config_debug_redacts_secrets_int_test() {
    let cfg: AwsSigV4StrategyConfig = AwsSigV4StrategyBuilder::new()
        .with_access_key_id(["AKIA", "IOSFODNN7EXAMPLE"].concat())
        .with_secret_access_key(["wJalrXUtnFEMI/K7MDENG", "/bPxRfiCYEXAMPLEKEY"].concat())
        .with_region("eu-west-1")
        .with_service("ec2")
        .build_config()
        .expect("builder must succeed");

    let debug_str = format!("{cfg:?}");
    let key_id = ["AKIA", "IOSFODNN7EXAMPLE"].concat();

    assert!(
        !debug_str.contains(key_id.as_str()),
        "Debug must not expose access_key_id; got: {debug_str}"
    );
    assert!(
        !debug_str.contains("wJalrXUtnFEMI"),
        "Debug must not expose secret_access_key; got: {debug_str}"
    );
    assert!(
        debug_str.contains("<redacted>"),
        "Debug must contain '<redacted>' placeholder; got: {debug_str}"
    );
}

/// @covers: AwsSigV4StrategyConfig fields
/// The `region` and `service` fields must be accessible and hold the
/// values supplied to the builder.
#[test]
fn auth_struct_aws_sig_v4_strategy_config_public_fields_accessible_int_test() {
    let cfg: AwsSigV4StrategyConfig = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("AKIA")
        .with_secret_access_key("secret")
        .with_region("ap-southeast-1")
        .with_service("dynamodb")
        .build_config()
        .expect("builder must succeed");

    assert_eq!(
        cfg.region, "ap-southeast-1",
        "region must match the value set on the builder"
    );
    assert_eq!(
        cfg.service, "dynamodb",
        "service must match the value set on the builder"
    );
}

/// @covers: AwsSigV4StrategyConfig session_token
/// `session_token` must be `None` when not supplied to the builder.
#[test]
fn auth_struct_aws_sig_v4_strategy_config_session_token_absent_by_default_int_test() {
    let cfg: AwsSigV4StrategyConfig = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("AKIA")
        .with_secret_access_key("secret")
        .with_region("us-west-2")
        .with_service("s3")
        .build_config()
        .expect("builder must succeed");

    assert!(
        cfg.session_token.is_none(),
        "session_token must be None when not set on the builder"
    );
}

/// @covers: AwsSigV4StrategyConfig Debug session_token
/// Debug output for a config without session token must show `<none>`.
#[test]
fn auth_struct_aws_sig_v4_strategy_config_debug_session_token_none_shows_placeholder_int_test() {
    let cfg: AwsSigV4StrategyConfig = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("AKIA")
        .with_secret_access_key("secret")
        .with_region("us-west-2")
        .with_service("s3")
        .build_config()
        .expect("builder must succeed");

    let debug_str = format!("{cfg:?}");
    assert!(
        debug_str.contains("<none>"),
        "Debug must show '<none>' for absent session_token; got: {debug_str}"
    );
}
