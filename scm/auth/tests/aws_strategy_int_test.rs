//! Integration test for `api/strategy/aws/aws_strategy.rs`.
//! @covers: src/api/strategy/aws/aws_strategy.rs

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::AwsSigV4StrategyBuilder;

/// @covers: AwsStrategy
/// Confirms the builder produces a valid config object with required fields.
#[test]
fn auth_struct_aws_strategy_builder_produces_config_int_test() {
    let cfg = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("AKID")
        .with_secret_access_key("SEC")
        .with_region("us-east-1")
        .with_service("s3")
        .build_config()
        .expect("all required fields set");
    assert_eq!(cfg.region, "us-east-1");
    assert_eq!(cfg.service, "s3");
    assert!(cfg.session_token.is_none());
}

/// @covers: AwsStrategy
/// Confirms that missing required fields produce an error rather than panicking.
#[test]
fn auth_struct_aws_strategy_builder_missing_required_field_returns_error_int_test() {
    let result = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("AKID")
        // secret_access_key intentionally omitted
        .with_region("us-east-1")
        .with_service("s3")
        .build_config();
    assert!(
        result.is_err(),
        "missing secret_access_key must fail build_config"
    );
}
