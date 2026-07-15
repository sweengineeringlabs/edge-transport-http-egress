//! Integration tests for `AwsSigV4StrategyConfigBuilder`.

use edge_transport_http_egress_auth::{AwsSigV4StrategyConfig, AwsSigV4StrategyConfigBuilder};

/// @covers: AwsSigV4StrategyConfigBuilder::new
#[test]
fn auth_struct_aws_sig_v4_strategy_config_builder_new_constructs_empty_builder_int_test() {
    let _builder = AwsSigV4StrategyConfigBuilder::new();
}

/// @covers: AwsSigV4StrategyConfigBuilder::build_config
#[test]
fn auth_struct_aws_sig_v4_strategy_config_builder_build_config_all_required_fields_succeeds_int_test(
) {
    let result: Result<AwsSigV4StrategyConfig, _> = AwsSigV4StrategyConfigBuilder::new()
        .with_access_key_id("AKIA")
        .with_secret_access_key("secret")
        .with_region("us-east-1")
        .with_service("s3")
        .build_config();
    assert!(
        result.is_ok(),
        "all required fields must produce Ok; got: {result:?}"
    );
}

/// @covers: AwsSigV4StrategyConfigBuilder::build_config
#[test]
fn auth_struct_aws_sig_v4_strategy_config_builder_build_config_missing_region_fails_int_test() {
    let result = AwsSigV4StrategyConfigBuilder::new()
        .with_access_key_id("AKIA")
        .with_secret_access_key("secret")
        .with_service("s3")
        .build_config();
    assert!(result.is_err(), "missing region must produce Err");
}
