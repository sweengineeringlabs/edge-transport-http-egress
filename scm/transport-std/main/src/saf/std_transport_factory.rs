//! The standard implementation of the transport construction contract.

use std::sync::Arc;

use edge_transport_http_egress_transport_api::{
    AlwaysValidConfig, ApplicationConfigBuilder, HttpConfig, HttpEgress, HttpEgressBuildError,
    HttpStream, TransportFactory, Validator,
};
use swe_observ_metrics::MetricsProvider;

use super::http_transport_svc::HttpTransportSvc;

/// Builds the standard reqwest-backed transport and its helpers.
#[derive(Debug, Clone, Copy, Default)]
pub struct StdTransportFactory;

impl TransportFactory for StdTransportFactory {
    fn create_config_builder(&self) -> ApplicationConfigBuilder {
        HttpTransportSvc::create_config_builder()
    }

    fn create_validator(&self) -> Box<dyn Validator> {
        Box::new(AlwaysValidConfig)
    }

    fn plain_http_egress(
        &self,
        config: HttpConfig,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        HttpTransportSvc::plain_http_egress(config)
    }

    fn plain_http_stream(
        &self,
        config: HttpConfig,
    ) -> Result<Box<dyn HttpStream>, HttpEgressBuildError> {
        HttpTransportSvc::plain_http_stream(config)
    }

    fn egress_from_middleware_client(
        &self,
        client: reqwest_middleware::ClientWithMiddleware,
        config: HttpConfig,
    ) -> Box<dyn HttpEgress> {
        HttpTransportSvc::egress_from_middleware_client(client, config)
    }

    fn observe_http_egress(
        &self,
        inner: Box<dyn HttpEgress>,
        provider: Arc<dyn MetricsProvider>,
    ) -> Box<dyn HttpEgress> {
        HttpTransportSvc::observe_http_egress(inner, provider)
    }

    fn validate_http_config(&self, config: &HttpConfig) -> Result<(), String> {
        HttpTransportSvc::validate_http_config(config)
    }

    fn validate(&self, validator: &dyn Validator) -> Result<(), String> {
        HttpTransportSvc::validate(validator)
    }

    fn configure_http_builder(
        &self,
        builder: reqwest::ClientBuilder,
        config: &HttpConfig,
    ) -> reqwest::ClientBuilder {
        HttpTransportSvc::configure_http_builder(builder, config)
    }
}
