//! `impl Validator for DefaultValidator` — loadbalancer config validation.

use crate::api::{ConfigValidationRequest, LoadbalancerMiddlewareError, Validator};

/// Default [`Validator`] implementation for
/// [`LoadbalancerConfig`](crate::api::LoadbalancerConfig).
pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(
        &self,
        request: ConfigValidationRequest,
    ) -> Result<(), LoadbalancerMiddlewareError> {
        let config = request.config;
        if config.backends.is_empty() {
            return Err(LoadbalancerMiddlewareError::InvalidConfig(
                "loadbalancer.backends must not be empty".to_string(),
            ));
        }
        for backend in &config.backends {
            if backend.url.is_empty() {
                return Err(LoadbalancerMiddlewareError::InvalidConfig(
                    "every backend must have a non-empty url".to_string(),
                ));
            }
            if backend.weight == 0 {
                return Err(LoadbalancerMiddlewareError::InvalidConfig(
                    "every backend must have weight >= 1; a weight of 0 removes it from selection"
                        .to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::LoadbalancerConfig;
    use swe_edge_loadbalancer::{BackendConfig, Strategy};

    fn config_with(backends: Vec<BackendConfig>) -> LoadbalancerConfig {
        LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends,
        }
    }

    fn backend(url: &str, weight: u32) -> BackendConfig {
        BackendConfig {
            url: url.to_string(),
            weight,
        }
    }

    /// @covers: validate
    #[test]
    fn test_validate_passes_for_valid_config() {
        let cfg = config_with(vec![backend("https://api.test", 1)]);
        assert!(DefaultValidator
            .validate(ConfigValidationRequest { config: cfg })
            .is_ok());
        // Sibling negative case in the same test: an otherwise-valid config
        // with backends removed must fail.
        let empty = config_with(vec![]);
        assert!(DefaultValidator
            .validate(ConfigValidationRequest { config: empty })
            .is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_empty_backends() {
        let err = DefaultValidator
            .validate(ConfigValidationRequest {
                config: config_with(vec![]),
            })
            .unwrap_err();
        assert!(
            matches!(err, LoadbalancerMiddlewareError::InvalidConfig(ref m) if m.contains("must not be empty")),
            "{err}"
        );
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_empty_url() {
        let err = DefaultValidator
            .validate(ConfigValidationRequest {
                config: config_with(vec![backend("", 1)]),
            })
            .unwrap_err();
        assert!(
            matches!(err, LoadbalancerMiddlewareError::InvalidConfig(ref m) if m.contains("non-empty url")),
            "{err}"
        );
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_zero_weight() {
        let err = DefaultValidator
            .validate(ConfigValidationRequest {
                config: config_with(vec![backend("https://api.test", 0)]),
            })
            .unwrap_err();
        assert!(
            matches!(err, LoadbalancerMiddlewareError::InvalidConfig(ref m) if m.contains("weight >= 1")),
            "{err}"
        );
    }
}
