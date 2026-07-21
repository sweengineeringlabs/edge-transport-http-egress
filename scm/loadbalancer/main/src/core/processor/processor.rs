//! `impl Processor for LoadbalancerSvcProcessor` — this crate's `Processor`
//! identity, plus its factory-method surface (config builder, config
//! validation, and layer construction).
//!
//! The type is declared in `api/`; the logic lives here in `core/` so the
//! declaration carries no dependency on `core/`.

use crate::api::{
    ConfigValidationRequest, DescribeRequest, DescribeResponse, LoadbalancerConfig,
    LoadbalancerLayerPoolMetrics, LoadbalancerMiddlewareError, LoadbalancerSvcProcessor, Processor,
    Validator,
};
use crate::core::DefaultValidator;

impl Processor for LoadbalancerSvcProcessor {
    fn describe(
        &self,
        _request: DescribeRequest,
    ) -> Result<DescribeResponse, LoadbalancerMiddlewareError> {
        Ok(DescribeResponse {
            value: env!("CARGO_PKG_NAME").to_string(),
        })
    }
}

impl LoadbalancerSvcProcessor {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Validate a [`LoadbalancerConfig`] and build a
    /// [`LoadbalancerLayerPoolMetrics`] from it.
    ///
    /// # Errors
    ///
    /// - [`LoadbalancerMiddlewareError::InvalidConfig`] — validation failed
    ///   (empty backend list, empty URL, or zero-weight backend).
    /// - [`LoadbalancerMiddlewareError::PoolBuildFailed`] — pool construction failed.
    pub fn build_layer(
        config: LoadbalancerConfig,
    ) -> Result<LoadbalancerLayerPoolMetrics, LoadbalancerMiddlewareError> {
        DefaultValidator.validate(ConfigValidationRequest {
            config: config.clone(),
        })?;
        LoadbalancerLayerPoolMetrics::new(config)
    }

    /// Validate a [`LoadbalancerConfig`] without constructing a layer.
    ///
    /// # Errors
    ///
    /// Returns [`LoadbalancerMiddlewareError::InvalidConfig`] if the config is
    /// malformed.
    pub fn validate_config(config: &LoadbalancerConfig) -> Result<(), LoadbalancerMiddlewareError> {
        DefaultValidator.validate(ConfigValidationRequest {
            config: config.clone(),
        })
    }
}
