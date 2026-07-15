//! SAF factory methods on [`HttpTransportSvc`](crate::HttpTransportSvc) for assembling [`HttpEgress`](crate::HttpEgress) instances.

#[cfg(feature = "oauth")]
use edge_transport_http_egress_oauth::OAuthBuilderOps as _;
use std::sync::Arc;
use std::time::Duration;

use reqwest_middleware::ClientBuilder;
use swe_observ_metrics::MetricsProvider;

use crate::api::error::HttpEgressBuildError;
use crate::api::traits::Validator as _;
use crate::api::traits::{HttpEgress, HttpStream};
use crate::api::types::{HttpConfig, HttpTransportSvc};
use crate::core::{DefaultHttpEgress, MetricsHttpEgress};

impl HttpTransportSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build an [`HttpEgress`](crate::HttpEgress) whose optional middleware are activated by the
    /// consumer's configuration (ADR-006 config-driven activation): a feature is
    /// wired **iff** its `[section]` is present in the loaded config.
    ///
    /// Config-drives `[auth]`, `[tls]`, `[retry]`, `[rate]`, `[breaker]`,
    /// `[cache]`, and `[cassette]` — present ⇒ the feature is wired; absent (or
    /// `enabled = false`) ⇒ it is **omitted from the chain**, not added as a
    /// no-op (zero overhead when disabled).
    ///
    /// `[auth]` wires the static auth strategy. OAuth token-refresh auth is a
    /// runtime `token_source` (a trait object), not a config section, so it is
    /// not activated here — supply it through the explicit `http_egress` /
    /// `http_egress_oauth` factories.
    ///
    /// ```toml
    /// # enabling TLS is all the consumer writes:
    /// [tls]
    /// kind = "pem"
    /// path = "/etc/certs/client.pem"
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`HttpEgressBuildError::Config`] if a section fails to load or
    /// validate, or the relevant middleware build error if assembly fails.
    pub fn http_egress_from_config(
        loader: &swe_edge_configbuilder::SectionLoaderImpl,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        #[cfg(feature = "auth")]
        use swe_edge_configbuilder::{FeatureState, OptionalSection as _};

        let http_cfg = HttpConfig::default();
        let client = Self::reqwest_client_from_config(loader, &http_cfg)?;
        let mut builder = ClientBuilder::new(client);

        // [auth] present ⇒ add the static auth layer. OAuth token-refresh is a
        // runtime token_source — use `http_egress_from_config_with_oauth` for it.
        #[cfg(feature = "auth")]
        if let FeatureState::Enabled(auth_cfg) =
            edge_transport_http_egress_auth::AuthConfig::load_optional(loader)?
        {
            let auth = edge_transport_http_egress_auth::AuthSvc::build_auth_middleware(auth_cfg)?;
            builder = builder.with(auth);
        }

        builder = Self::with_optional_layers(loader, builder)?;
        Ok(Box::new(DefaultHttpEgress::new(
            builder.build(),
            http_cfg.base_url,
            http_cfg.max_response_bytes,
        )))
    }

    /// Like [`http_egress_from_config`](HttpTransportSvc::http_egress_from_config), but the auth slot is an **OAuth
    /// token-refresh** layer built from `token_source` instead of the static
    /// `[auth]` section. This is how OAuth — which cannot be expressed in TOML
    /// (the token source is a runtime trait object) — is combined with the
    /// config-driven middleware stack. The `[tls]`/`[retry]`/`[rate]`/
    /// `[breaker]`/`[cache]`/`[cassette]` sections are honoured as usual; the
    /// `[auth]` section is ignored (OAuth takes the auth slot).
    ///
    /// # Errors
    ///
    /// Returns [`HttpEgressBuildError::Config`] if a section fails to load or
    /// validate, [`HttpEgressBuildError::OAuth`] if the OAuth layer cannot be
    /// built, or the relevant middleware build error.
    #[cfg(feature = "oauth")]
    pub fn http_egress_from_config_with_oauth(
        loader: &swe_edge_configbuilder::SectionLoaderImpl,
        token_source: Arc<dyn edge_transport_http_egress_oauth::OAuthTokenSource>,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        let http_cfg = HttpConfig::default();
        let client = Self::reqwest_client_from_config(loader, &http_cfg)?;
        let mut builder = ClientBuilder::new(client);

        // OAuth occupies the auth slot, replacing any static [auth] section.
        let oauth = edge_transport_http_egress_oauth::OAuthSvc::builder()
            .with_token_source(token_source)
            .build()?;
        builder = builder.with(oauth);

        builder = Self::with_optional_layers(loader, builder)?;
        Ok(Box::new(DefaultHttpEgress::new(
            builder.build(),
            http_cfg.base_url,
            http_cfg.max_response_bytes,
        )))
    }

    /// Dry-run the config-driven egress: load every optional `[section]` into a
    /// [`FeatureRegistry`] and return a [`FeatureSummary`] of what would
    /// activate — without building any middleware. Log this at startup so
    /// operators see exactly which features are on (and why); it is the
    /// visibility guardrail against silent config-driven activation.
    ///
    /// Mirrors the section set of [`http_egress_from_config`](HttpTransportSvc::http_egress_from_config): `[auth]`,
    /// `[tls]`, `[retry]`, `[rate]`, `[breaker]`, `[cache]`, `[cassette]`.
    ///
    /// # Errors
    ///
    /// Returns [`HttpEgressBuildError::Config`] if a present section fails to
    /// parse or validate.
    ///
    /// [`FeatureRegistry`]: swe_edge_configbuilder::FeatureRegistry
    /// [`FeatureSummary`]: swe_edge_configbuilder::FeatureSummary
    // With zero middleware features, no section is registered: `loader` goes
    // unread and `registry` is never mutated before `summary()`.
    #[cfg_attr(
        not(any(
            feature = "auth",
            feature = "tls",
            feature = "retry",
            feature = "rate",
            feature = "breaker",
            feature = "cache",
            feature = "cassette"
        )),
        allow(unused_variables, unused_mut)
    )]
    pub fn preflight(
        loader: &swe_edge_configbuilder::SectionLoaderImpl,
    ) -> Result<swe_edge_configbuilder::FeatureSummary, HttpEgressBuildError> {
        let mut registry = swe_edge_configbuilder::FeatureRegistry::new();
        #[cfg(feature = "auth")]
        registry.load::<edge_transport_http_egress_auth::AuthConfig>(loader)?;
        #[cfg(feature = "tls")]
        registry.load::<edge_transport_http_egress_tls::TlsConfig>(loader)?;
        #[cfg(feature = "retry")]
        registry.load::<edge_transport_http_egress_retry::RetryConfig>(loader)?;
        #[cfg(feature = "rate")]
        registry.load::<edge_transport_http_egress_rate::RateConfig>(loader)?;
        #[cfg(feature = "breaker")]
        registry.load::<edge_transport_http_egress_breaker::BreakerConfig>(loader)?;
        #[cfg(feature = "cache")]
        registry.load::<edge_transport_http_egress_cache::CacheConfig>(loader)?;
        #[cfg(feature = "cassette")]
        registry.load::<edge_transport_http_egress_cassette::CassetteConfig>(loader)?;
        Ok(registry.summary())
    }

    /// Build an [`HttpEgress`](crate::HttpEgress) using the SWE-shipped defaults for every
    /// middleware layer (pass-through auth, no TLS, sensible retry/rate/breaker
    /// policies from each crate's `config/application.toml`).
    pub fn default_http_egress() -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        Ok(Box::new(Self::build_default_egress(
            HttpConfig::default(),
            #[cfg(feature = "cassette")]
            edge_transport_http_egress_cassette::CassetteConfig::default(),
            #[cfg(feature = "cassette")]
            "default",
        )?))
    }

    /// Build a fully-assembled [`HttpEgress`](crate::HttpEgress) using the provided [`HttpConfig`]
    /// and SWE defaults for every middleware layer.
    ///
    /// Same middleware stack as [`default_http_egress`](HttpTransportSvc::default_http_egress) but with caller-supplied
    /// transport settings (base URL, timeouts, headers, etc.).
    pub fn default_http_egress_with_config(
        http: HttpConfig,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        Ok(Box::new(Self::build_default_egress(
            http,
            #[cfg(feature = "cassette")]
            edge_transport_http_egress_cassette::CassetteConfig::disabled(),
            #[cfg(feature = "cassette")]
            "unused",
        )?))
    }

    /// Wrap any [`HttpEgress`](crate::HttpEgress) with per-call metrics observation.
    ///
    /// Consumers call this after any of the factory functions to add observability
    /// without changing how the outbound is configured:
    ///
    /// ```rust,ignore
    /// let outbound = HttpTransportSvc::observe_http_egress(default_http_egress()?, metrics_provider);
    /// ```
    /// Wrap any [`HttpEgress`](crate::HttpEgress) with per-call metrics observation.
    ///
    /// Consumers call this after any of the factory functions to add observability
    /// without changing how the outbound is configured:
    ///
    /// ```rust,ignore
    /// let outbound = HttpTransportSvc::observe_http_egress(default_http_egress()?, metrics_provider);
    /// ```
    pub fn observe_http_egress(
        inner: Box<dyn HttpEgress>,
        provider: Arc<dyn MetricsProvider>,
    ) -> Box<dyn HttpEgress> {
        let arc_inner: Arc<dyn HttpEgress> = Arc::from(inner);
        Box::new(MetricsHttpEgress::new(arc_inner, provider))
    }

    /// Build a fully-assembled [`HttpStream`](crate::HttpStream) using the SWE defaults.
    ///
    /// Returns the same default middleware stack as [`default_http_egress`](HttpTransportSvc::default_http_egress)
    /// but typed as [`HttpStream`](crate::HttpStream), so callers can use SSE and WebSocket
    /// features without importing or naming the concrete type.
    pub fn default_http_stream_outbound() -> Result<Box<dyn HttpStream>, HttpEgressBuildError> {
        Ok(Box::new(Self::build_default_egress(
            HttpConfig::default(),
            #[cfg(feature = "cassette")]
            edge_transport_http_egress_cassette::CassetteConfig::default(),
            #[cfg(feature = "cassette")]
            "default",
        )?))
    }

    /// Build a minimal [`HttpEgress`](crate::HttpEgress) from just an [`HttpConfig`] — no middleware layers.
    ///
    /// All `HttpConfig` fields are honoured: `timeout_secs`, `connect_timeout_secs`,
    /// `user_agent`, `follow_redirects`, `max_redirects`, `default_headers`, and
    /// `max_response_bytes`.  Useful for integration tests and simple deployments
    /// that do not need the full auth/retry/rate/breaker/cache/cassette stack.
    pub fn plain_http_egress(
        config: HttpConfig,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        let mut cb = reqwest::Client::builder();
        cb = cb.timeout(Duration::from_secs(config.timeout_secs));
        cb = cb.connect_timeout(Duration::from_secs(config.connect_timeout_secs));
        if let Some(ua) = &config.user_agent {
            cb = cb.user_agent(ua);
        }
        if config.follow_redirects {
            cb = cb.redirect(reqwest::redirect::Policy::limited(
                config.max_redirects as usize,
            ));
        } else {
            cb = cb.redirect(reqwest::redirect::Policy::none());
        }
        if !config.default_headers.is_empty() {
            let mut map = reqwest::header::HeaderMap::new();
            for (k, v) in &config.default_headers {
                if let (Ok(name), Ok(val)) = (
                    reqwest::header::HeaderName::from_bytes(k.as_bytes()),
                    reqwest::header::HeaderValue::from_str(v),
                ) {
                    map.insert(name, val);
                }
            }
            cb = cb.default_headers(map);
        }
        let client = reqwest_middleware::ClientBuilder::new(cb.build()?).build();
        Ok(Box::new(DefaultHttpEgress::new(
            client,
            config.base_url,
            config.max_response_bytes,
        )))
    }

    /// Build a minimal [`HttpEgress`](crate::HttpEgress) from an [`HttpConfig`] and a runtime OAuth
    /// token source — no config-builder, no other middleware layers.
    ///
    /// This fills the gap between [`plain_http_egress`] (no auth) and
    /// [`http_egress_from_config_with_oauth`] (full config-builder stack): a
    /// caller who already has an `HttpConfig` and an `Arc<dyn OAuthTokenSource>`
    /// but does not use the config-builder can assemble an OAuth-enabled egress
    /// through this factory without manually wrapping `DefaultHttpEgress`.
    ///
    /// All `HttpConfig` fields are honoured (timeouts, user-agent, redirect policy,
    /// default headers, `max_response_bytes`). The OAuth bearer token is injected
    /// per request via the `OAuthSvc` middleware layer.
    ///
    /// # Errors
    ///
    /// Returns [`HttpEgressBuildError::Reqwest`] if the underlying reqwest client
    /// cannot be built, or [`HttpEgressBuildError::OAuth`] if the `OAuthSvc`
    /// layer fails to initialise.
    ///
    /// [`plain_http_egress`]: HttpTransportSvc::plain_http_egress
    /// [`http_egress_from_config_with_oauth`]: HttpTransportSvc::http_egress_from_config_with_oauth
    #[cfg(feature = "oauth")]
    pub fn plain_http_egress_with_oauth(
        config: HttpConfig,
        token_source: Arc<dyn edge_transport_http_egress_oauth::OAuthTokenSource>,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        let client = Self::configure_http_builder(reqwest::Client::builder(), &config).build()?;
        let oauth = edge_transport_http_egress_oauth::OAuthSvc::builder()
            .with_token_source(token_source)
            .build()?;
        let mw_client = ClientBuilder::new(client).with(oauth).build();
        Ok(Box::new(DefaultHttpEgress::new(
            mw_client,
            config.base_url,
            config.max_response_bytes,
        )))
    }

    /// Validate that an [`HttpConfig`] value is well-formed.
    ///
    /// Returns `Ok(())` when the config is valid, or a human-readable error
    /// message explaining what field is invalid and what the expected constraint is.
    pub fn validate_http_config(config: &HttpConfig) -> Result<(), String> {
        use crate::core::validator::{DefaultValidator, HttpConfigValidator};
        DefaultValidator::new(HttpConfigValidator::new(config)).validate()
    }

    /// Validate any value that implements the `Validator` trait.
    ///
    /// This is the generic gateway to the `Validator` contract — consumers who
    /// implement `Validator` on their own types can call this to run validation
    /// through the SAF boundary without holding a direct reference to the trait.
    pub fn validate<V: crate::api::traits::Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }

    /// Build the reqwest client for a config-driven egress: apply the `[tls]`
    /// section if present, then the [`HttpConfig`] transport settings.
    // Without the `tls` feature, `loader` is unread (no `[tls]` section to load).
    #[cfg_attr(not(feature = "tls"), allow(unused_variables))]
    fn reqwest_client_from_config(
        loader: &swe_edge_configbuilder::SectionLoaderImpl,
        http_cfg: &HttpConfig,
    ) -> Result<reqwest::Client, HttpEgressBuildError> {
        #[cfg(feature = "tls")]
        use swe_edge_configbuilder::{FeatureState, OptionalSection as _};

        #[cfg_attr(not(feature = "tls"), allow(unused_mut))]
        let mut cb = reqwest::Client::builder();
        // [tls] present ⇒ build and apply the TLS layer; absent ⇒ no TLS layer.
        #[cfg(feature = "tls")]
        if let FeatureState::Enabled(tls_cfg) =
            edge_transport_http_egress_tls::TlsConfig::load_optional(loader)?
        {
            let tls = edge_transport_http_egress_tls::HttpTlsSvc::build_tls_layer(tls_cfg)?;
            cb = tls.apply_to(cb)?;
        }
        cb = Self::configure_http_builder(cb, http_cfg);
        Ok(cb.build()?)
    }

    /// Append the non-auth optional middleware — `[retry]`, `[rate]`,
    /// `[breaker]`, `[cache]`, `[cassette]` — each added via `.with(..)` only
    /// when its section is present; a disabled section adds nothing.
    // With none of these five features, no layer is appended: `loader` is unread
    // and `builder` is returned unmutated.
    #[cfg_attr(
        not(any(
            feature = "retry",
            feature = "rate",
            feature = "breaker",
            feature = "cache",
            feature = "cassette"
        )),
        allow(unused_variables, unused_mut)
    )]
    fn with_optional_layers(
        loader: &swe_edge_configbuilder::SectionLoaderImpl,
        mut builder: ClientBuilder,
    ) -> Result<ClientBuilder, HttpEgressBuildError> {
        #[cfg(any(
            feature = "retry",
            feature = "rate",
            feature = "breaker",
            feature = "cache",
            feature = "cassette"
        ))]
        use swe_edge_configbuilder::{FeatureState, OptionalSection as _};

        #[cfg(feature = "retry")]
        if let FeatureState::Enabled(retry_cfg) =
            edge_transport_http_egress_retry::RetryConfig::load_optional(loader)?
        {
            builder = builder.with(
                edge_transport_http_egress_retry::HttpRetrySvc::build_retry_layer(retry_cfg)?,
            );
        }
        #[cfg(feature = "rate")]
        if let FeatureState::Enabled(rate_cfg) =
            edge_transport_http_egress_rate::RateConfig::load_optional(loader)?
        {
            builder = builder
                .with(edge_transport_http_egress_rate::HttpRateSvc::build_rate_layer(rate_cfg)?);
        }
        #[cfg(feature = "breaker")]
        if let FeatureState::Enabled(breaker_cfg) =
            edge_transport_http_egress_breaker::BreakerConfig::load_optional(loader)?
        {
            builder = builder.with(
                edge_transport_http_egress_breaker::HttpBreakerSvc::build_breaker_layer(
                    breaker_cfg,
                )?,
            );
        }
        #[cfg(feature = "cache")]
        if let FeatureState::Enabled(cache_cfg) =
            edge_transport_http_egress_cache::CacheConfig::load_optional(loader)?
        {
            builder = builder.with(
                edge_transport_http_egress_cache::HttpCacheSvc::build_cache_layer(cache_cfg)?,
            );
        }
        #[cfg(feature = "cassette")]
        if let FeatureState::Enabled(cassette_cfg) =
            edge_transport_http_egress_cassette::CassetteConfig::load_optional(loader)?
        {
            builder = builder.with(
                edge_transport_http_egress_cassette::HttpCassetteSvc::build_cassette_layer(
                    cassette_cfg,
                    "default",
                )?,
            );
        }
        Ok(builder)
    }

    /// Build a [`DefaultHttpEgress`] with the full SWE-default middleware stack —
    /// pass-through auth, default retry/rate/breaker/cache, the supplied cassette
    /// config, and no client TLS. Shared by the `default_*` factories; built
    /// directly (no `assemble`) so each layer can be feature-gated independently.
    #[cfg_attr(
        not(any(
            feature = "auth",
            feature = "retry",
            feature = "rate",
            feature = "breaker",
            feature = "cache",
            feature = "cassette"
        )),
        allow(unused_mut)
    )]
    fn build_default_egress(
        http_cfg: HttpConfig,
        #[cfg(feature = "cassette")]
        cassette_cfg: edge_transport_http_egress_cassette::CassetteConfig,
        #[cfg(feature = "cassette")] cassette_name: &str,
    ) -> Result<DefaultHttpEgress, HttpEgressBuildError> {
        let mut cb = reqwest::Client::builder();
        #[cfg(feature = "tls")]
        {
            let tls =
                edge_transport_http_egress_tls::HttpTlsSvc::build_tls_layer(Default::default())?;
            cb = tls.apply_to(cb)?;
        }
        cb = Self::configure_http_builder(cb, &http_cfg);

        let mut builder = ClientBuilder::new(cb.build()?);
        #[cfg(feature = "auth")]
        {
            builder =
                builder.with(
                    edge_transport_http_egress_auth::AuthSvc::build_auth_middleware(
                        Default::default(),
                    )?,
                );
        }
        #[cfg(feature = "retry")]
        {
            builder = builder.with(
                edge_transport_http_egress_retry::HttpRetrySvc::build_retry_layer(
                    Default::default(),
                )?,
            );
        }
        #[cfg(feature = "rate")]
        {
            builder = builder.with(
                edge_transport_http_egress_rate::HttpRateSvc::build_rate_layer(Default::default())?,
            );
        }
        #[cfg(feature = "breaker")]
        {
            builder = builder.with(
                edge_transport_http_egress_breaker::HttpBreakerSvc::build_breaker_layer(
                    Default::default(),
                )?,
            );
        }
        #[cfg(feature = "cache")]
        {
            builder = builder.with(
                edge_transport_http_egress_cache::HttpCacheSvc::build_cache_layer(
                    Default::default(),
                )?,
            );
        }
        #[cfg(feature = "cassette")]
        {
            builder = builder.with(
                edge_transport_http_egress_cassette::HttpCassetteSvc::build_cassette_layer(
                    cassette_cfg,
                    cassette_name,
                )?,
            );
        }

        Ok(DefaultHttpEgress::new(
            builder.build(),
            http_cfg.base_url,
            http_cfg.max_response_bytes,
        ))
    }

    /// Apply [`HttpConfig`] transport settings — timeouts, user-agent, redirect
    /// policy, and default headers — to a reqwest client builder. Shared by the
    /// config-driven and default-stack builders.
    fn configure_http_builder(
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
