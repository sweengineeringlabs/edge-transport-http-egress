//! The construction contract of the outbound HTTP transport.

use std::sync::Arc;

use swe_observ_metrics::MetricsProvider;

use crate::{
    ApplicationConfigBuilder, HttpConfig, HttpEgress, HttpEgressBuildError, HttpStream, Validator,
};

/// Builds every consumer-facing transport object, so no implementation type is public.
pub trait TransportFactory: Send + Sync {
    /// A builder for the application config loader, seeded with this package name and version.
    fn create_config_builder(&self) -> ApplicationConfigBuilder;

    /// A [`Validator`] that accepts every config (the default no-op validation).
    fn create_validator(&self) -> Box<dyn Validator>;

    /// A plain HTTP client for `config`, with no middleware.
    ///
    /// # Errors
    /// Returns an [`HttpEgressBuildError`] when the underlying client cannot be built.
    fn plain_http_egress(
        &self,
        config: HttpConfig,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError>;

    /// A plain HTTP streaming client for `config`, with no middleware.
    ///
    /// # Errors
    /// Returns an [`HttpEgressBuildError`] when the underlying client cannot be built.
    fn plain_http_stream(
        &self,
        config: HttpConfig,
    ) -> Result<Box<dyn HttpStream>, HttpEgressBuildError>;

    /// Wrap an already-built middleware client (retry, cache, ... composed by the caller).
    fn egress_from_middleware_client(
        &self,
        client: reqwest_middleware::ClientWithMiddleware,
        config: HttpConfig,
    ) -> Box<dyn HttpEgress>;

    /// Decorate `inner` so each call is reported to `provider`.
    fn observe_http_egress(
        &self,
        inner: Box<dyn HttpEgress>,
        provider: Arc<dyn MetricsProvider>,
    ) -> Box<dyn HttpEgress>;

    /// Validate an [`HttpConfig`].
    ///
    /// # Errors
    /// Returns the validation failure message.
    fn validate_http_config(&self, config: &HttpConfig) -> Result<(), String>;

    /// Run any [`Validator`].
    ///
    /// # Errors
    /// Returns the validation failure message.
    fn validate(&self, validator: &dyn Validator) -> Result<(), String>;

    /// Apply `config` to a `reqwest` client builder (timeouts, user agent, redirects, headers).
    fn configure_http_builder(
        &self,
        builder: reqwest::ClientBuilder,
        config: &HttpConfig,
    ) -> reqwest::ClientBuilder;
}
