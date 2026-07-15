//! Default impl of [`Processor`](crate::api::traits::Processor).

use crate::api::traits::{Processor, Validator};
use swe_edge_loadbalancer::LoadbalancerConfig;

/// Default `Processor` implementation. `pub(crate)` — consumers never touch
/// this type directly; they go through `saf/`.
#[derive(Debug)]
pub(crate) struct DefaultLoadbalancerMiddleware {
    config: LoadbalancerConfig,
}

impl DefaultLoadbalancerMiddleware {
    pub(crate) fn new(config: LoadbalancerConfig) -> Self {
        Self { config }
    }
}

impl Processor for DefaultLoadbalancerMiddleware {
    fn describe(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}

impl Validator for DefaultLoadbalancerMiddleware {
    fn validate(&self) -> Result<(), String> {
        if self.config.backends.is_empty() {
            return Err("loadbalancer.backends must not be empty".to_string());
        }
        for backend in &self.config.backends {
            if backend.url.is_empty() {
                return Err("every backend must have a non-empty url".to_string());
            }
            if backend.weight == 0 {
                return Err(
                    "every backend must have weight >= 1; a weight of 0 removes it from selection"
                        .to_string(),
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_loadbalancer::{BackendConfig, Strategy};

    fn config_with(backends: Vec<BackendConfig>) -> LoadbalancerConfig {
        LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends,
        }
    }

    fn healthy_backend(url: &str) -> BackendConfig {
        BackendConfig {
            url: url.to_string(),
            weight: 1,
        }
    }

    /// @covers: new
    #[test]
    fn test_new_constructs_and_stores_config() {
        let cfg = config_with(vec![healthy_backend("https://api.test")]);
        let d = DefaultLoadbalancerMiddleware::new(cfg);
        let dbg = format!("{d:?}");
        assert!(dbg.contains("DefaultLoadbalancerMiddleware"), "{dbg}");
    }

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_name() {
        let cfg = config_with(vec![healthy_backend("https://api.test")]);
        let d = DefaultLoadbalancerMiddleware::new(cfg);
        assert_eq!(d.describe(), "edge-transport-http-egress-loadbalancer");
    }

    /// @covers: validate
    #[test]
    fn test_validate_passes_for_valid_config() {
        let cfg = config_with(vec![healthy_backend("https://api.test")]);
        let d = DefaultLoadbalancerMiddleware::new(cfg);
        assert!(d.validate().is_ok(), "valid config must pass");
    }

    /// @covers: validate
    #[test]
    fn test_validate_fails_for_empty_backends() {
        let cfg = config_with(vec![]);
        let d = DefaultLoadbalancerMiddleware::new(cfg);
        let err = d.validate().unwrap_err();
        assert!(err.contains("must not be empty"), "{err}");
    }

    /// @covers: validate
    #[test]
    fn test_validate_fails_for_empty_url() {
        let cfg = config_with(vec![BackendConfig {
            url: "".to_string(),
            weight: 1,
        }]);
        let d = DefaultLoadbalancerMiddleware::new(cfg);
        let err = d.validate().unwrap_err();
        assert!(err.contains("non-empty url"), "{err}");
    }

    /// @covers: validate
    #[test]
    fn test_validate_fails_for_zero_weight() {
        let cfg = config_with(vec![BackendConfig {
            url: "https://api.test".to_string(),
            weight: 0,
        }]);
        let d = DefaultLoadbalancerMiddleware::new(cfg);
        let err = d.validate().unwrap_err();
        assert!(err.contains("weight >= 1"), "{err}");
    }
}
