//! Default impl of [`Processor`](crate::api::traits::Processor).
//!
//! Holds a resolved [`RateConfig`](crate::api::types::RateConfig)
//! and answers `describe()`.

use crate::api::traits::Processor;
use crate::api::traits::Validator;
use crate::api::types::RateConfig;

/// Default `Processor` implementation. `pub(crate)` — consumers
/// never touch this type directly; they go through `saf/`.
#[derive(Debug)]
pub(crate) struct DefaultHttpRate {
    config: RateConfig,
}

impl DefaultHttpRate {
    /// Construct from a resolved config.
    pub(crate) fn new(config: RateConfig) -> Self {
        Self { config }
    }
}

impl Processor for DefaultHttpRate {
    fn describe(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}

impl Validator for DefaultHttpRate {
    fn validate(&self) -> Result<(), String> {
        if self.config.tokens_per_second == 0 {
            return Err(
                "tokens_per_second must be >= 1; a rate of 0 would block all requests".to_string(),
            );
        }
        if self.config.burst_capacity == 0 {
            return Err(
                "burst_capacity must be >= 1; a burst of 0 would deny every request".to_string(),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_constructs_and_stores_config() {
        let cfg = RateConfig::default();
        let d = DefaultHttpRate::new(cfg);
        let dbg = format!("{d:?}");
        assert!(dbg.contains("DefaultHttpRate"), "debug output: {dbg}");
    }

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_name() {
        let cfg = RateConfig::default();
        let d = DefaultHttpRate::new(cfg);
        assert_eq!(d.describe(), "edge-transport-http-egress-rate");
    }

    /// @covers: validate
    #[test]
    fn test_validate_passes_for_valid_config() {
        let cfg = RateConfig::default();
        let d = DefaultHttpRate::new(cfg);
        assert!(d.validate().is_ok(), "default config must validate");
    }

    /// @covers: validate
    #[test]
    fn test_validate_fails_for_zero_tokens_per_second() {
        let cfg = RateConfig {
            tokens_per_second: 0,
            burst_capacity: 10,
            per_host: false,
        };
        let d = DefaultHttpRate::new(cfg);
        let result = d.validate();
        assert!(
            result.is_err(),
            "zero tokens_per_second must fail validation"
        );
        let msg = result.expect_err("expected error");
        assert!(
            msg.contains("tokens_per_second"),
            "error must mention the failing field; got: {msg}"
        );
    }

    /// @covers: validate
    #[test]
    fn test_validate_fails_for_zero_burst_capacity() {
        let cfg = RateConfig {
            tokens_per_second: 10,
            burst_capacity: 0,
            per_host: false,
        };
        let d = DefaultHttpRate::new(cfg);
        let result = d.validate();
        assert!(result.is_err(), "zero burst_capacity must fail validation");
        let msg = result.expect_err("expected error");
        assert!(
            msg.contains("burst_capacity"),
            "error must mention the failing field; got: {msg}"
        );
    }
}
