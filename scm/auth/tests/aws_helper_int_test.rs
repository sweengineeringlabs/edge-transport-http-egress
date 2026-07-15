//! Integration test for `api/strategy/aws/helper.rs`.
//! @covers: src/api/strategy/aws/helper.rs

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_auth::AwsSigV4StrategyBuilder;

/// @covers: Helper
/// Confirms the builder's optional session_token field wires through correctly.
/// This exercises the aws helper layer by configuring a full STS credential set.
#[test]
fn auth_struct_aws_helper_builder_with_session_token_int_test() {
    let cfg = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("AKID")
        .with_secret_access_key("SECRET")
        .with_session_token("STS_TOKEN")
        .with_region("eu-west-1")
        .with_service("sts")
        .build_config()
        .expect("all fields set");
    assert!(cfg.session_token.is_some(), "session_token must be set");
    assert_eq!(cfg.region, "eu-west-1");
    assert_eq!(cfg.service, "sts");
}

/// @covers: Helper
/// Confirms the builder debug output redacts credentials.
#[test]
fn auth_struct_aws_helper_config_debug_redacts_secrets_int_test() {
    let cfg = AwsSigV4StrategyBuilder::new()
        .with_access_key_id("SUPER_SECRET_KEY_ID")
        .with_secret_access_key("SUPER_SECRET_ACCESS_KEY")
        .with_region("ap-southeast-1")
        .with_service("execute-api")
        .build_config()
        .expect("all required fields set");
    let debug = format!("{cfg:?}");
    assert!(
        !debug.contains("SUPER_SECRET_KEY_ID"),
        "access key must not appear in debug"
    );
    assert!(
        !debug.contains("SUPER_SECRET_ACCESS_KEY"),
        "secret key must not appear in debug"
    );
    assert!(
        debug.contains("redacted"),
        "debug must show redaction marker"
    );
}
