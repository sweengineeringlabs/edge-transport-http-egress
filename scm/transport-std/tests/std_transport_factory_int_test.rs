//! Integration tests for `StdTransportFactory`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport_api::{
    ConfigRequest, HttpConfig, TransportFactory, ValidatableHttpConfig, ValidateRequest,
    ValidationError, Validator,
};
use edge_transport_http_egress_transport_std::StdTransportFactory;

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_accepts_a_default_config_happy() {
    assert!(StdTransportFactory
        .validate_http_config(&HttpConfig::default())
        .is_ok());
    // Paired with a rejected config, so the Ok above is a verdict and not a type that cannot be Err.
    assert!(StdTransportFactory
        .validate_http_config(&HttpConfig::default().with_timeout(0))
        .is_err());
}

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_rejects_a_zero_connect_timeout_error() {
    let config = HttpConfig {
        connect_timeout_secs: 0,
        ..HttpConfig::default()
    };
    let message = StdTransportFactory
        .validate_http_config(&config)
        .expect_err("a zero connect timeout must be rejected");
    assert!(message.contains("connect_timeout_secs"), "got: {message}");
}

/// @covers: validate
#[test]
fn test_validate_runs_any_validator_and_reports_its_verdict_error() {
    let invalid = ValidatableHttpConfig {
        config: HttpConfig::default().with_timeout(0),
    };
    let message = StdTransportFactory
        .validate(&invalid)
        .expect_err("a zero timeout must be rejected");
    assert!(message.contains("timeout_secs"), "got: {message}");

    let valid = ValidatableHttpConfig {
        config: HttpConfig::default(),
    };
    assert!(StdTransportFactory.validate(&valid).is_ok());
}

/// @covers: validate
#[test]
fn test_validate_through_a_trait_object_matches_the_concrete_call_edge() {
    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(&self, _request: ValidateRequest) -> Result<(), ValidationError> {
            Err(ValidationError::Invalid("always rejects".to_string()))
        }
    }
    let boxed: Box<dyn Validator> = Box::new(AlwaysInvalid);
    assert!(StdTransportFactory
        .validate(boxed.as_ref())
        .expect_err("rejected")
        .contains("always rejects"));
}

/// @covers: configure_http_builder
#[test]
fn test_configure_http_builder_produces_a_buildable_client_happy() {
    let config = HttpConfig::default().with_header("x-trace", "abc");
    let client = StdTransportFactory
        .configure_http_builder(reqwest::Client::builder(), &config)
        .build();
    let client = client.expect("a valid config must build a client");
    assert!(
        format!("{client:?}").contains("x-trace"),
        "the default header must reach the client; got: {client:?}"
    );
}

/// @covers: configure_http_builder
#[test]
fn test_configure_http_builder_ignores_an_invalid_header_name_edge() {
    let config = HttpConfig::default()
        .with_header("bad header name", "v")
        .with_header("x-kept", "1");
    let client = StdTransportFactory
        .configure_http_builder(reqwest::Client::builder(), &config)
        .build()
        .expect("an invalid header is skipped, not fatal");
    let debug = format!("{client:?}");
    assert!(!debug.contains("bad header name"), "got: {debug}");
    assert!(
        debug.contains("x-kept"),
        "valid headers must survive; got: {debug}"
    );
}

/// @covers: plain_http_egress
#[test]
fn test_plain_http_egress_and_stream_carry_the_given_config_happy() {
    let config = HttpConfig::with_base_url("http://127.0.0.1:9").with_timeout(7);
    let egress = StdTransportFactory
        .plain_http_egress(config.clone())
        .expect("a default-shaped config must build");
    let carried = egress
        .config(ConfigRequest)
        .expect("config is infallible")
        .config;
    assert_eq!(carried.timeout_secs, 7);
    assert_eq!(carried.base_url, config.base_url);

    // The streaming client is built from the same config; building is the observable result.
    let _stream = StdTransportFactory
        .plain_http_stream(config)
        .expect("the same config must build a streaming client");
}
