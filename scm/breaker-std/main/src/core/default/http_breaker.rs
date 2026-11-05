//! `impl Processor for HttpBreakerSvcProcessor` — this crate's `Processor`
//! identity, plus its factory-method surface.
//!
//! `HttpBreakerSvcProcessor` is otherwise a static factory namespace (see
//! `saf/`); this impl gives it a genuine role as a trait implementor,
//! matching the `Processor` contract declared in the `-api` package.

use configbuilder::ConfigBuilder;
use edge_transport_http_egress_breaker_api::{
    BreakerConfig, BreakerError, DescribeRequest, DescribeResponse, Processor,
};

use super::http_breaker_svc_processor::HttpBreakerSvcProcessor;
use crate::core::breaker::layer::BreakerLayerBreakerMetrics;

impl Processor for HttpBreakerSvcProcessor {
    fn describe(&self, _request: DescribeRequest) -> Result<DescribeResponse, BreakerError> {
        Ok(DescribeResponse {
            value: "http-breaker".to_string(),
        })
    }
}

impl HttpBreakerSvcProcessor {
    /// The application name the config builder is seeded with. configbuilder uses it as a directory
    /// segment when it searches for configuration, so it stays the name of the package this
    /// middleware had before the `-api`/`-std` split; seeding the `-std` package name would change
    /// where existing deployments' configuration is found.
    const CONFIG_APPLICATION_NAME: &'static str = "edge-transport-http-egress-breaker";

    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> configbuilder::ConfigBuilderImpl {
        let mut b = configbuilder::ConfigLoaderFactory::create_config_builder();
        b = b.with_name(Self::CONFIG_APPLICATION_NAME);
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build a [`BreakerLayerBreakerMetrics`] from a caller-supplied [`BreakerConfig`].
    pub fn build_breaker_layer(
        config: BreakerConfig,
    ) -> Result<BreakerLayerBreakerMetrics, BreakerError> {
        let layer = BreakerLayerBreakerMetrics::new(config);
        Ok(layer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_transport_http_egress_breaker_api::{BreakerMetrics, FailureThresholdRequest};

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_label() {
        let resp = HttpBreakerSvcProcessor
            .describe(DescribeRequest)
            .expect("describe is infallible");
        assert_eq!(resp.value, "http-breaker");
    }

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_seeds_name_and_version_inline() {
        let builder = HttpBreakerSvcProcessor::create_config_builder();
        assert_eq!(builder.name(), "edge-transport-http-egress-breaker");
        assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
    }

    /// @covers: build_breaker_layer
    #[test]
    fn test_failure_threshold_reads_configured_value_inline() {
        let cfg = BreakerConfig {
            failure_threshold: 9,
            ..BreakerConfig::default()
        };
        let layer = HttpBreakerSvcProcessor::build_breaker_layer(cfg).expect("build ok");
        let threshold = layer
            .failure_threshold(FailureThresholdRequest)
            .expect("reading the configured threshold must succeed");
        assert_eq!(threshold.value, 9);
    }

    /// @covers: build_breaker_layer
    #[test]
    fn test_build_breaker_layer_succeeds_with_default_config_inline() {
        HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default())
            .expect("default config must build");
    }
}
