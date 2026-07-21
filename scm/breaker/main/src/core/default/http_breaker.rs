//! `impl Processor for HttpBreakerSvcProcessor` — this crate's `Processor`
//! identity, plus its factory-method surface.
//!
//! `HttpBreakerSvcProcessor` is otherwise a static factory namespace (see
//! `saf/`); this impl gives it a genuine role as a trait implementor,
//! matching the `Processor` contract declared in `api/`.

use crate::api::{
    BreakerConfig, BreakerError, BreakerLayerBreakerMetrics, BreakerMetrics, DescribeRequest,
    DescribeResponse, FailureThresholdRequest, HttpBreakerSvcProcessor, Processor,
};

impl Processor for HttpBreakerSvcProcessor {
    fn describe(&self, _request: DescribeRequest) -> Result<DescribeResponse, BreakerError> {
        Ok(DescribeResponse {
            value: "http-breaker".to_string(),
        })
    }
}

impl HttpBreakerSvcProcessor {
    /// Returns the failure threshold configured on a [`BreakerLayerBreakerMetrics`].
    #[expect(
        clippy::expect_used,
        reason = "BreakerLayerBreakerMetrics's BreakerMetrics impl never returns Err — the Result \
                  shape is a structural api/ contract requirement, not a real failure mode"
    )]
    pub fn get_failure_threshold(layer: &BreakerLayerBreakerMetrics) -> u32 {
        layer
            .failure_threshold(FailureThresholdRequest)
            .expect("BreakerLayerBreakerMetrics::failure_threshold is infallible")
            .value
    }

    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
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

    /// Build a [`BreakerLayerBreakerMetrics`] that reports circuit-trip and recovery events
    /// back to a `BackendPool`.
    ///
    /// Requires the `loadbalancer` feature. When the circuit opens (trip) the
    /// layer reports `Outcome::Failure { reason: "circuit open" }` to the pool
    /// for the affected backend, removing it from rotation. When a half-open
    /// probe succeeds the layer reports `Outcome::Success`, restoring the
    /// backend.
    #[cfg(feature = "loadbalancer")]
    pub fn build_breaker_layer_with_pool(
        config: BreakerConfig,
        pool: std::sync::Arc<swe_edge_loadbalancer::BackendPoolInstance>,
    ) -> Result<BreakerLayerBreakerMetrics, BreakerError> {
        let layer = BreakerLayerBreakerMetrics::new_with_pool(config, pool);
        Ok(layer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_label() {
        let resp = HttpBreakerSvcProcessor
            .describe(DescribeRequest)
            .expect("describe is infallible");
        assert_eq!(resp.value, "http-breaker");
    }

    /// @covers: get_failure_threshold
    #[test]
    fn test_get_failure_threshold_reads_configured_value_inline() {
        let cfg = BreakerConfig {
            failure_threshold: 9,
            ..BreakerConfig::default()
        };
        let layer = HttpBreakerSvcProcessor::build_breaker_layer(cfg).expect("build ok");
        assert_eq!(HttpBreakerSvcProcessor::get_failure_threshold(&layer), 9);
    }

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_seeds_crate_name_inline() {
        let builder = HttpBreakerSvcProcessor::create_config_builder();
        assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
    }

    /// @covers: build_breaker_layer
    #[test]
    fn test_build_breaker_layer_succeeds_with_default_config_inline() {
        HttpBreakerSvcProcessor::build_breaker_layer(BreakerConfig::default())
            .expect("default config must build");
    }

    /// @covers: build_breaker_layer_with_pool
    #[cfg(feature = "loadbalancer")]
    #[test]
    fn test_build_breaker_layer_with_pool_succeeds_inline() {
        use swe_edge_loadbalancer::{
            build_backend_pool, BackendConfig, LoadbalancerConfig, Strategy,
        };
        let pool = std::sync::Arc::new(
            build_backend_pool(LoadbalancerConfig {
                strategy: Strategy::RoundRobin,
                backends: vec![BackendConfig {
                    url: "http://example.test".to_string(),
                    weight: 1,
                }],
            })
            .expect("pool build ok"),
        );
        HttpBreakerSvcProcessor::build_breaker_layer_with_pool(BreakerConfig::default(), pool)
            .expect("build with pool must succeed");
    }
}
