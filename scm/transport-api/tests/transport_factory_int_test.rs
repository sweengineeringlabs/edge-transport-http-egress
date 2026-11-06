//! Integration tests for the `TransportFactory` contract, through a test double: the trait must be
//! object safe and carry its arguments (config, validator, metrics provider) unchanged.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::{Arc, Mutex};

use edge_transport_http_egress_transport_api::{
    AlwaysValidConfig, ApplicationConfigBuilder, HttpConfig, HttpEgress, HttpEgressBuildError,
    HttpStream, TransportFactory, ValidatableHttpConfig, ValidateRequest, ValidationError,
    Validator,
};
use swe_observ_metrics::MetricsProvider;

/// A factory that builds nothing and records what it was handed.
#[derive(Default)]
struct RecordingFactory {
    seen_provider: Mutex<Option<Arc<dyn MetricsProvider>>>,
}

fn build_error() -> HttpEgressBuildError {
    // An unparsable URL makes reqwest fail at build time, without any network access.
    let err = reqwest::Client::new()
        .get("not a url")
        .build()
        .expect_err("an unparsable url must not build");
    HttpEgressBuildError::from(err)
}

impl TransportFactory for RecordingFactory {
    fn create_config_builder(&self) -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::new().with_name("recording-factory")
    }

    fn create_validator(&self) -> Box<dyn Validator> {
        Box::new(AlwaysValidConfig)
    }

    fn plain_http_egress(
        &self,
        _config: HttpConfig,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        Err(build_error())
    }

    fn plain_http_stream(
        &self,
        _config: HttpConfig,
    ) -> Result<Box<dyn HttpStream>, HttpEgressBuildError> {
        Err(build_error())
    }

    fn egress_from_middleware_client(
        &self,
        _client: reqwest_middleware::ClientWithMiddleware,
        _config: HttpConfig,
    ) -> Box<dyn HttpEgress> {
        unreachable!("this double never builds a client")
    }

    fn observe_http_egress(
        &self,
        inner: Box<dyn HttpEgress>,
        provider: Arc<dyn MetricsProvider>,
    ) -> Box<dyn HttpEgress> {
        *self.seen_provider.lock().unwrap() = Some(provider);
        inner
    }

    fn validate_http_config(&self, config: &HttpConfig) -> Result<(), String> {
        self.validate(&ValidatableHttpConfig {
            config: config.clone(),
        })
    }

    fn validate(&self, validator: &dyn Validator) -> Result<(), String> {
        validator
            .validate(ValidateRequest)
            .map_err(|e| e.to_string())
    }

    fn configure_http_builder(
        &self,
        builder: reqwest::ClientBuilder,
        _config: &HttpConfig,
    ) -> reqwest::ClientBuilder {
        builder
    }
}

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_accepts_a_default_config_happy() {
    let factory: &dyn TransportFactory = &RecordingFactory::default();
    assert!(factory.validate_http_config(&HttpConfig::default()).is_ok());
    // Paired with a rejected config, so the Ok above is a verdict and not a type that cannot be Err.
    assert!(factory
        .validate_http_config(&HttpConfig::default().with_timeout(0))
        .is_err());
}

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_rejects_a_zero_timeout_error() {
    let factory: &dyn TransportFactory = &RecordingFactory::default();
    let message = factory
        .validate_http_config(&HttpConfig::default().with_timeout(0))
        .expect_err("a zero timeout must be rejected");
    assert!(message.contains("timeout_secs"), "got: {message}");
}

/// @covers: validate
#[test]
fn test_validate_reports_the_validator_message_error() {
    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(&self, _request: ValidateRequest) -> Result<(), ValidationError> {
            Err(ValidationError::Invalid("always rejects".to_string()))
        }
    }
    let factory: &dyn TransportFactory = &RecordingFactory::default();
    let message = factory
        .validate(&AlwaysInvalid)
        .expect_err("the validator verdict must be returned");
    assert!(message.contains("always rejects"), "got: {message}");
    assert!(factory.validate(&AlwaysValidConfig).is_ok());
}

/// @covers: plain_http_egress
#[test]
fn test_plain_http_egress_surfaces_the_build_error_error() {
    let factory: &dyn TransportFactory = &RecordingFactory::default();
    let result = factory.plain_http_egress(HttpConfig::default());
    assert!(
        matches!(result, Err(HttpEgressBuildError::Reqwest(_))),
        "a reqwest failure must surface as HttpEgressBuildError::Reqwest"
    );
    assert!(factory.plain_http_stream(HttpConfig::default()).is_err());
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_is_seeded_by_the_implementation_happy() {
    let factory: &dyn TransportFactory = &RecordingFactory::default();
    assert_eq!(factory.create_config_builder().name(), "recording-factory");
    assert!(factory.create_validator().validate(ValidateRequest).is_ok());
}
