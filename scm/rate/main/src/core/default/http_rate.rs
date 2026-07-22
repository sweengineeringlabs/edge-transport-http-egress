//! `impl Processor for HttpRateSvcProcessor` — this crate's `Processor`
//! identity, plus its factory-method surface.
//!
//! `HttpRateSvcProcessor` is otherwise a static factory namespace (see
//! `saf/`); this impl gives it a genuine role as a trait implementor,
//! matching the `Processor` contract declared in `api/`.

use crate::api::{
    ConfigValidationRequest, DescribeRequest, DescribeResponse, HttpRateSvcProcessor, Processor,
    RateConfig, RateError, RateLayerRateMetrics, Validator,
};
use crate::core::rate::default_validator::DefaultValidator;

impl Processor for HttpRateSvcProcessor {
    fn describe(&self, _request: DescribeRequest) -> Result<DescribeResponse, RateError> {
        Ok(DescribeResponse {
            value: "http-rate".to_string(),
        })
    }
}

impl HttpRateSvcProcessor {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
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
        assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
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
