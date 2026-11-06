//! `StdResilientLayerComposer` — the real implementation of
//! [`edge_transport_http_egress_resilient_api::ResilientLayerComposer`].
//!
//! Lifted from `transport`'s `saf/transport_svc.rs` (`with_optional_layers`/
//! `build_default_egress`), unchanged internally — same config-section-driven
//! activation, same `retry → rate → breaker → cache → cassette` ordering.
//! Only the location and error type changed, per ADR-006.

use configbuilder::{FeatureState, OptionalSection as _};
use reqwest_middleware::ClientBuilder;

use edge_transport_http_egress_resilient_api::{
    ResilientConfigRequest, ResilientDefaultsRequest, ResilientError, ResilientLayerComposer,
};

/// The only implementor of [`ResilientLayerComposer`] — a zero-field marker type,
/// matching the `HttpRetrySvc`/`HttpBreakerSvcProcessor`-style unit-struct
/// convention already used by every layer crate this composes.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdResilientLayerComposer;

impl ResilientLayerComposer for StdResilientLayerComposer {
    fn preflight(
        &self,
        loader: &configbuilder::SectionLoaderImpl,
    ) -> Result<configbuilder::FeatureSummary, configbuilder::ConfigError> {
        Self::run_preflight(loader)
    }

    fn apply_from_config(
        &self,
        request: ResilientConfigRequest<'_>,
    ) -> Result<ClientBuilder, ResilientError> {
        let ResilientConfigRequest {
            mut builder,
            loader,
        } = request;

        if let FeatureState::Enabled(retry_cfg) =
            edge_transport_http_egress_retry_api::RetryConfig::load_optional(loader)
                .map_err(|e| ResilientError::Config(e.to_string()))?
        {
            builder = builder.with_arc(
                {
                    use edge_transport_http_egress_retry_api::RetryFactory as _;
                    edge_transport_http_egress_retry_std::StdRetryFactory
                        .create_processor()
                        .decorate(edge_transport_http_egress_retry_api::DecorateRequest {
                            config: retry_cfg,
                        })
                        .map_err(|e| ResilientError::Retry(e.to_string()))?
                }
                .layer,
            );
        }
        if let FeatureState::Enabled(rate_cfg) =
            edge_transport_http_egress_rate_api::RateConfig::load_optional(loader)
                .map_err(|e| ResilientError::Config(e.to_string()))?
        {
            builder = builder.with_arc(
                edge_transport_http_egress_rate_api::RateFactory::build_rate_layer(
                    &edge_transport_http_egress_rate_std::StdRateFactory,
                    rate_cfg,
                )
                .map_err(|e| ResilientError::Rate(e.to_string()))?,
            );
        }
        if let FeatureState::Enabled(breaker_cfg) =
            edge_transport_http_egress_breaker_api::BreakerConfig::load_optional(loader)
                .map_err(|e| ResilientError::Config(e.to_string()))?
        {
            builder = builder.with_arc(
                edge_transport_http_egress_breaker_api::BreakerFactory::build_breaker_layer(
                    &edge_transport_http_egress_breaker_std::StdBreakerFactory,
                    breaker_cfg,
                )
                .map_err(|e| ResilientError::Breaker(e.to_string()))?,
            );
        }
        if let FeatureState::Enabled(cache_cfg) =
            edge_transport_http_egress_cache_api::CacheConfig::load_optional(loader)
                .map_err(|e| ResilientError::Config(e.to_string()))?
        {
            builder = builder.with_arc(
                edge_transport_http_egress_cache_api::CacheFactory::build_cache_layer(
                    &edge_transport_http_egress_cache_std::StdCacheFactory,
                    cache_cfg,
                )
                .map_err(|e| ResilientError::Cache(e.to_string()))?,
            );
        }
        if let FeatureState::Enabled(cassette_cfg) =
            edge_transport_http_egress_cassette_api::CassetteConfig::load_optional(loader)
                .map_err(|e| ResilientError::Config(e.to_string()))?
        {
            builder = builder.with_arc(
                edge_transport_http_egress_cassette_api::CassetteFactory::build_cassette_layer(
                    &edge_transport_http_egress_cassette_std::StdCassetteFactory,
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
        request: ResilientDefaultsRequest,
    ) -> Result<ClientBuilder, ResilientError> {
        let ResilientDefaultsRequest {
            mut builder,
            cassette,
            cassette_name,
        } = request;

        builder = builder.with_arc(
            {
                use edge_transport_http_egress_retry_api::RetryFactory as _;
                edge_transport_http_egress_retry_std::StdRetryFactory
                    .create_processor()
                    .decorate(edge_transport_http_egress_retry_api::DecorateRequest {
                        config: Default::default(),
                    })
                    .map_err(|e| ResilientError::Retry(e.to_string()))?
            }
            .layer,
        );
        builder = builder.with_arc(
            edge_transport_http_egress_rate_api::RateFactory::build_rate_layer(
                &edge_transport_http_egress_rate_std::StdRateFactory,
                Default::default(),
            )
            .map_err(|e| ResilientError::Rate(e.to_string()))?,
        );
        builder = builder.with_arc(
            edge_transport_http_egress_breaker_api::BreakerFactory::build_breaker_layer(
                &edge_transport_http_egress_breaker_std::StdBreakerFactory,
                Default::default(),
            )
            .map_err(|e| ResilientError::Breaker(e.to_string()))?,
        );
        builder = builder.with_arc(
            edge_transport_http_egress_cache_api::CacheFactory::build_cache_layer(
                &edge_transport_http_egress_cache_std::StdCacheFactory,
                Default::default(),
            )
            .map_err(|e| ResilientError::Cache(e.to_string()))?,
        );
        builder = builder.with_arc(
            edge_transport_http_egress_cassette_api::CassetteFactory::build_cassette_layer(
                &edge_transport_http_egress_cassette_std::StdCassetteFactory,
                cassette,
                &cassette_name,
            )
            .map_err(|e| ResilientError::Cassette(e.to_string()))?,
        );

        Ok(builder)
    }
}
