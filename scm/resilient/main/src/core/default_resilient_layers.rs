//! `DefaultResilientLayers` — the real implementation of
//! [`edge_transport_http_egress_resilient_port::ResilientLayers`].
//!
//! Lifted from `transport`'s `saf/transport_svc.rs` (`with_optional_layers`/
//! `build_default_egress`), unchanged internally — same config-section-driven
//! activation, same `retry → rate → breaker → cache → cassette` ordering.
//! Only the location and error type changed, per ADR-006.

use reqwest_middleware::ClientBuilder;
use swe_edge_configbuilder::{FeatureState, OptionalSection as _};

use edge_transport_http_egress_resilient_port::{
    ApplyDefaultsRequest, ApplyFromConfigRequest, ResilientError, ResilientLayers,
};

/// The only implementor of [`ResilientLayers`] — a zero-field marker type,
/// matching the `HttpRetrySvc`/`HttpBreakerSvcProcessor`-style unit-struct
/// convention already used by every layer crate this composes.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultResilientLayers;

impl ResilientLayers for DefaultResilientLayers {
    fn apply_from_config(
        &self,
        request: ApplyFromConfigRequest<'_>,
    ) -> Result<ClientBuilder, ResilientError> {
        let ApplyFromConfigRequest {
            mut builder,
            loader,
        } = request;

        if let FeatureState::Enabled(retry_cfg) =
            edge_transport_http_egress_retry::RetryConfig::load_optional(loader)
                .map_err(|e| ResilientError::Config(e.to_string()))?
        {
            builder = builder.with(
                {
                    use edge_transport_http_egress_retry::Processor as _;
                    edge_transport_http_egress_retry::HttpRetrySvc
                        .decorate(edge_transport_http_egress_retry::DecorateRequest {
                            config: retry_cfg,
                        })
                        .map_err(|e| ResilientError::Retry(e.to_string()))?
                }
                .layer,
            );
        }
        if let FeatureState::Enabled(rate_cfg) =
            edge_transport_http_egress_rate::RateConfig::load_optional(loader)
                .map_err(|e| ResilientError::Config(e.to_string()))?
        {
            builder = builder.with(
                edge_transport_http_egress_rate::HttpRateSvcProcessor::build_rate_layer(rate_cfg)
                    .map_err(|e| ResilientError::Rate(e.to_string()))?,
            );
        }
        if let FeatureState::Enabled(breaker_cfg) =
            edge_transport_http_egress_breaker::BreakerConfig::load_optional(loader)
                .map_err(|e| ResilientError::Config(e.to_string()))?
        {
            builder = builder.with(
                edge_transport_http_egress_breaker::HttpBreakerSvcProcessor::build_breaker_layer(
                    breaker_cfg,
                )
                .map_err(|e| ResilientError::Breaker(e.to_string()))?,
            );
        }
        if let FeatureState::Enabled(cache_cfg) =
            edge_transport_http_egress_cache::CacheConfig::load_optional(loader)
                .map_err(|e| ResilientError::Config(e.to_string()))?
        {
            builder = builder.with(
                edge_transport_http_egress_cache::HttpCacheSvcProcessor::build_cache_layer(
                    cache_cfg,
                )
                .map_err(|e| ResilientError::Cache(e.to_string()))?,
            );
        }
        if let FeatureState::Enabled(cassette_cfg) =
            edge_transport_http_egress_cassette::CassetteConfig::load_optional(loader)
                .map_err(|e| ResilientError::Config(e.to_string()))?
        {
            builder = builder.with(
                edge_transport_http_egress_cassette::HttpCassetteSvc::build_cassette_layer(
                    cassette_cfg,
                    "default",
                )
                .map_err(|e| ResilientError::Cassette(e.to_string()))?,
            );
        }

        Ok(builder)
    }

    fn apply_defaults(
        &self,
        request: ApplyDefaultsRequest,
    ) -> Result<ClientBuilder, ResilientError> {
        let ApplyDefaultsRequest {
            mut builder,
            cassette,
            cassette_name,
        } = request;

        builder = builder.with(
            {
                use edge_transport_http_egress_retry::Processor as _;
                edge_transport_http_egress_retry::HttpRetrySvc
                    .decorate(edge_transport_http_egress_retry::DecorateRequest {
                        config: Default::default(),
                    })
                    .map_err(|e| ResilientError::Retry(e.to_string()))?
            }
            .layer,
        );
        builder = builder.with(
            edge_transport_http_egress_rate::HttpRateSvcProcessor::build_rate_layer(
                Default::default(),
            )
            .map_err(|e| ResilientError::Rate(e.to_string()))?,
        );
        builder = builder.with(
            edge_transport_http_egress_breaker::HttpBreakerSvcProcessor::build_breaker_layer(
                Default::default(),
            )
            .map_err(|e| ResilientError::Breaker(e.to_string()))?,
        );
        builder = builder.with(
            edge_transport_http_egress_cache::HttpCacheSvcProcessor::build_cache_layer(
                Default::default(),
            )
            .map_err(|e| ResilientError::Cache(e.to_string()))?,
        );
        builder = builder.with(
            edge_transport_http_egress_cassette::HttpCassetteSvc::build_cassette_layer(
                cassette,
                &cassette_name,
            )
            .map_err(|e| ResilientError::Cassette(e.to_string()))?,
        );

        Ok(builder)
    }
}
