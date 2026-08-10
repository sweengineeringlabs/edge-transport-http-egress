//! SAF factory methods on [`HttpTransportSvc`](crate::HttpTransportSvc) for assembling bare
//! [`HttpEgress`](crate::HttpEgress) / [`HttpStream`](crate::HttpStream) instances.
//!
//! `transport` is a **bare HTTP primitive**: it builds a client from an [`HttpConfig`] and exposes
//! it through the `HttpEgress`/`HttpStream` contracts. It does **not** own composition of the
//! retry/rate/breaker/cache/cassette resilient chain, nor of auth/oauth/tls — those are assembled by
//! a composition root (e.g. `edge-bootstrap`'s `RuntimeBuilder`), which then hands its finished
//! `reqwest_middleware::ClientWithMiddleware` back through
//! [`egress_from_middleware_client`](HttpTransportSvc::egress_from_middleware_client). See ADR-005 /
//! ADR-006, edge-transport-http-egress#28.

use std::sync::Arc;
use std::time::Duration;

use swe_observ_metrics::MetricsProvider;

use crate::api::{
    ApplicationConfigBuilder, HttpConfig, HttpEgress, HttpEgressBuildError, HttpStream,
    HttpTransportSvc,
};
use crate::core::{DefaultHttpEgress, MetricsHttpEgress};

impl HttpTransportSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::new()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Build a bare [`HttpEgress`](crate::HttpEgress) from just an [`HttpConfig`] — no middleware
    /// layers.
    ///
    /// All `HttpConfig` fields are honoured: `timeout_secs`, `connect_timeout_secs`, `user_agent`,
    /// `follow_redirects`, `max_redirects`, `default_headers`, and `max_response_bytes`. This is the
    /// primitive — a composition root adds retry/rate/breaker/cache/cassette/auth/tls on top and
    /// re-wraps the result via
    /// [`egress_from_middleware_client`](HttpTransportSvc::egress_from_middleware_client).
    pub fn plain_http_egress(
        config: HttpConfig,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        let client = Self::build_plain_client(&config)?;
        Ok(Box::new(DefaultHttpEgress::new(client, config)))
    }

    /// Build a bare [`HttpStream`](crate::HttpStream) from just an [`HttpConfig`] — no middleware
    /// layers.
    ///
    /// The streaming counterpart of [`plain_http_egress`](HttpTransportSvc::plain_http_egress): the
    /// same bare client, typed as [`HttpStream`](crate::HttpStream) so callers can use SSE and
    /// WebSocket features without importing or naming the concrete type. As with egress, composing a
    /// resilient stream is a composition-root concern, not `transport`'s.
    pub fn plain_http_stream(
        config: HttpConfig,
    ) -> Result<Box<dyn HttpStream>, HttpEgressBuildError> {
        let client = Self::build_plain_client(&config)?;
        Ok(Box::new(DefaultHttpEgress::new(client, config)))
    }

    /// Wrap an already-built [`reqwest_middleware::ClientWithMiddleware`] as an
    /// [`HttpEgress`](crate::HttpEgress), applying the supplied [`HttpConfig`].
    ///
    /// This is the construction seam for a *composition root* (e.g. `edge-bootstrap`'s
    /// `RuntimeBuilder`) that assembles its own middleware chain — TLS at the reqwest layer, the
    /// `edge-transport-http-egress-resilient` decorator, caller-supplied auth/oauth — and then needs
    /// to expose the finished client through this crate's `HttpEgress` contract. `transport` provides
    /// the bare primitive ([`plain_http_egress`](HttpTransportSvc::plain_http_egress)) and this wrap
    /// seam; it does **not** own the composition of the layers themselves.
    ///
    /// Infallible: wrapping an already-built client cannot fail (any build error surfaced earlier,
    /// when the caller constructed the `reqwest_middleware::ClientWithMiddleware`).
    pub fn egress_from_middleware_client(
        client: reqwest_middleware::ClientWithMiddleware,
        config: HttpConfig,
    ) -> Box<dyn HttpEgress> {
        Box::new(DefaultHttpEgress::new(client, config))
    }

    /// Wrap any [`HttpEgress`](crate::HttpEgress) with per-call metrics observation.
    ///
    /// Consumers call this after any factory function to add observability without changing how the
    /// outbound is configured:
    ///
    /// ```rust,ignore
    /// let outbound = HttpTransportSvc::observe_http_egress(plain_http_egress(cfg)?, metrics_provider);
    /// ```
    pub fn observe_http_egress(
        inner: Box<dyn HttpEgress>,
        provider: Arc<dyn MetricsProvider>,
    ) -> Box<dyn HttpEgress> {
        let arc_inner: Arc<dyn HttpEgress> = Arc::from(inner);
        Box::new(MetricsHttpEgress::new(arc_inner, provider))
    }

    /// Validate that an [`HttpConfig`] value is well-formed.
    ///
    /// Returns `Ok(())` when the config is valid, or a human-readable error message explaining what
    /// field is invalid and what the expected constraint is.
    pub fn validate_http_config(config: &HttpConfig) -> Result<(), String> {
        use crate::api::ValidateRequest;
        use crate::api::Validator as _;
        use crate::core::validator::{DefaultValidator, HttpConfigValidator};
        DefaultValidator::new(HttpConfigValidator::new(config))
            .validate(ValidateRequest)
            .map_err(|e| e.to_string())
    }

    /// Validate any value that implements the `Validator` trait.
    ///
    /// This is the generic gateway to the `Validator` contract — consumers who implement `Validator`
    /// on their own types can call this to run validation through the SAF boundary without holding a
    /// direct reference to the trait.
    pub fn validate<V: crate::api::Validator>(v: &V) -> Result<(), String> {
        use crate::api::ValidateRequest;
        v.validate(ValidateRequest).map_err(|e| e.to_string())
    }

    /// Build the bare `reqwest_middleware::ClientWithMiddleware` shared by
    /// [`plain_http_egress`](HttpTransportSvc::plain_http_egress) and
    /// [`plain_http_stream`](HttpTransportSvc::plain_http_stream). No middleware layers are added —
    /// only the [`HttpConfig`] transport settings are applied to the underlying reqwest client.
    fn build_plain_client(
        config: &HttpConfig,
    ) -> Result<reqwest_middleware::ClientWithMiddleware, HttpEgressBuildError> {
        let cb = Self::configure_http_builder(reqwest::Client::builder(), config);
        Ok(reqwest_middleware::ClientBuilder::new(cb.build()?).build())
    }

    /// Apply an [`HttpConfig`]'s transport settings — timeouts, user-agent, redirect policy, and
    /// default headers — to a [`reqwest::ClientBuilder`].
    ///
    /// This is the **canonical `HttpConfig` → reqwest mapping**. `transport`'s own bare primitives
    /// ([`plain_http_egress`](HttpTransportSvc::plain_http_egress) /
    /// [`plain_http_stream`](HttpTransportSvc::plain_http_stream)) apply it internally; it is exposed
    /// so a *composition root* assembling its own middleware stack (TLS layer + resilient decorator +
    /// this mapping, then [`egress_from_middleware_client`](HttpTransportSvc::egress_from_middleware_client))
    /// applies the config through the same single implementation rather than re-deriving it. See
    /// edge-transport-http-egress#29.
    pub fn configure_http_builder(
        mut cb: reqwest::ClientBuilder,
        http_cfg: &HttpConfig,
    ) -> reqwest::ClientBuilder {
        cb = cb.timeout(Duration::from_secs(http_cfg.timeout_secs));
        cb = cb.connect_timeout(Duration::from_secs(http_cfg.connect_timeout_secs));
        if let Some(ua) = &http_cfg.user_agent {
            cb = cb.user_agent(ua);
        }
        if http_cfg.follow_redirects {
            cb = cb.redirect(reqwest::redirect::Policy::limited(
                http_cfg.max_redirects as usize,
            ));
        } else {
            cb = cb.redirect(reqwest::redirect::Policy::none());
        }
        if !http_cfg.default_headers.is_empty() {
            let mut map = reqwest::header::HeaderMap::new();
            for (k, v) in &http_cfg.default_headers {
                if let (Ok(name), Ok(val)) = (
                    reqwest::header::HeaderName::from_bytes(k.as_bytes()),
                    reqwest::header::HeaderValue::from_str(v),
                ) {
                    map.insert(name, val);
                }
            }
            cb = cb.default_headers(map);
        }
        cb
    }
}
