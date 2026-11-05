//! `impl Processor for HttpRateSvcProcessor` — this crate's `Processor`
//! identity, plus its factory-method surface.
//!
//! `HttpRateSvcProcessor` is otherwise a static factory namespace (see
//! `saf/`); this impl gives it a genuine role as a trait implementor,
//! matching the `Processor` contract declared in `api/`.

use super::http_rate_svc_processor::HttpRateSvcProcessor;
use crate::core::rate::default_validator::DefaultValidator;
use crate::core::rate::RateLayerRateMetrics;
use configbuilder::ConfigBuilder;
use edge_transport_http_egress_rate_api::{
    ConfigValidationRequest, DescribeRequest, DescribeResponse, Processor, RateConfig, RateError,
    Validator,
};

impl Processor for HttpRateSvcProcessor {
    fn describe(&self, _request: DescribeRequest) -> Result<DescribeResponse, RateError> {
        Ok(DescribeResponse {
            value: "http-rate".to_string(),
        })
    }
}

impl HttpRateSvcProcessor {
    /// The application name the config builder is seeded with. configbuilder uses it as a directory
    /// segment when it searches for configuration, so it stays the name of the package this
    /// middleware had before the `-api`/`-std` split; seeding the `-std` package name would change
    /// where existing deployments' configuration is found.
    const CONFIG_APPLICATION_NAME: &'static str = "edge-transport-http-egress-rate";

    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> configbuilder::ConfigBuilderImpl {
        let mut b = configbuilder::ConfigLoaderFactory::create_config_builder();
        b = b.with_name(Self::CONFIG_APPLICATION_NAME);
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Validate a [`RateConfig`] and build a [`RateLayerRateMetrics`] from it.
    ///
    /// Returns `Err` if the config fails validation (e.g. zero token rate).
    pub fn build_rate_layer(config: RateConfig) -> Result<RateLayerRateMetrics, RateError> {
        DefaultValidator.validate(ConfigValidationRequest {
            config: config.clone(),
        })?;
        let layer = RateLayerRateMetrics::new(config);
        Ok(layer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_label() {
        let resp = HttpRateSvcProcessor
            .describe(DescribeRequest)
            .expect("describe is infallible");
        assert_eq!(resp.value, "http-rate");
    }

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_seeds_crate_name_inline() {
        let builder = HttpRateSvcProcessor::create_config_builder();
        assert_eq!(builder.name(), "edge-transport-http-egress-rate");
    }

    /// @covers: build_rate_layer
    #[test]
    fn test_build_rate_layer_succeeds_with_default_config_inline() {
        HttpRateSvcProcessor::build_rate_layer(RateConfig::default())
            .expect("default config must build");
    }

    /// @covers: build_rate_layer
    #[test]
    fn test_build_rate_layer_rejects_zero_rate_inline() {
        let bad = RateConfig {
            tokens_per_second: 0,
            burst_capacity: 10,
            per_host: false,
        };
        assert!(
            HttpRateSvcProcessor::build_rate_layer(bad).is_err(),
            "zero token rate must be rejected"
        );
    }
}
