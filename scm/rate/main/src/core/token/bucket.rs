//! Per-host token bucket.
//!
//! Tokens refill at `tokens_per_second` up to `burst_capacity`.
//! Each request tries to consume one token; if none available,
//! the caller waits until a token is ready (calculated from the
//! refill rate).
//!
//! Known, accepted `core_structs_have_trait` gap: `TokenBucket` is a pure
//! internal helper — constructed and consumed only by
//! `core::rate::layer::RateLayerRateMetrics`'s middleware dispatch, never
//! returned from `saf/` or exposed as a trait object. This matches the
//! rule's own documented edge case ("Pure internal helper structs used only
//! within core/ (never returned from saf/) are exempt"), but the static
//! checker still flags it. Backing this with a new `pub trait` in api/
//! would misrepresent an implementation detail as a public contract, so
//! this is left unresolved and documented rather than hacked around —
//! mirrors the `saf_no_inherent_impl` gap accepted elsewhere in this
//! codebase.

use std::time::{Duration, Instant};

use crate::api::RateConfig;

/// Token bucket state. Not thread-safe on its own — wrap in a
/// mutex for concurrent use (the middleware does this via moka
/// + tokio::sync::Mutex).
#[derive(Debug)]
pub(crate) struct TokenBucket {
    /// Current token count. Fractional — tokens accumulate
    /// linearly even when refill rate isn't a whole number.
    tokens: f64,
    /// When we last refilled the bucket. Used to compute how
    /// many tokens have accumulated since.
    last_refill: Instant,
}

impl TokenBucket {
    /// Construct a full bucket (consumers shouldn't be
    /// artificially throttled on startup).
    pub(crate) fn new(config: &RateConfig) -> Self {
        Self {
            tokens: config.burst_capacity as f64,
            last_refill: Instant::now(),
        }
    }

    /// Refill (based on elapsed time) then try to consume one token.
    ///
    /// Returns `Ok(())` if a token was available and consumed.
    /// Returns `Err(wait)` if the bucket is empty; `wait` is the
    /// time until one token will be available.
    pub(crate) fn try_consume(&mut self, config: &RateConfig) -> Result<(), Duration> {
        self.refill(config);
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            Ok(())
        } else {
            let deficit = 1.0 - self.tokens;
            let secs_until_one = deficit / config.tokens_per_second as f64;
            Err(Duration::from_secs_f64(secs_until_one))
        }
    }

    /// Refill tokens based on elapsed time since last refill.
    /// Caps at `burst_capacity`.
    fn refill(&mut self, config: &RateConfig) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        let added = elapsed.as_secs_f64() * config.tokens_per_second as f64;
        self.tokens = (self.tokens + added).min(config.burst_capacity as f64);
        self.last_refill = now;
    }

    #[cfg(test)]
    pub(crate) fn tokens(&self) -> f64 {
        self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> RateConfig {
        RateConfig::from_config(
            r#"
                tokens_per_second = 10
                burst_capacity = 20
                per_host = true
            "#,
        )
        .expect("test config must parse")
    }

    /// @covers: new
    #[test]
    fn test_new_initialises_to_burst_capacity() {
        let cfg = test_config();
        let b = TokenBucket::new(&cfg);
        assert_eq!(b.tokens(), cfg.burst_capacity as f64);
    }

    /// @covers: new
    #[test]
    fn test_new_full_starts_at_burst_capacity() {
        let cfg = test_config();
        let b = TokenBucket::new(&cfg);
        assert_eq!(b.tokens(), 20.0);
    }

    /// @covers: try_consume
    #[test]
    fn test_try_consume_consumes_one_token_on_success() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        let before = b.tokens();
        b.try_consume(&cfg)
            .expect("fresh bucket must yield a token");
        assert!(b.tokens() < before, "one token must be consumed");
    }

    /// @covers: try_consume
    #[test]
    fn test_try_consume_returns_wait_on_exhausted_bucket() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        for _ in 0..20 {
            b.try_consume(&cfg).expect("drain must succeed");
        }
        match b.try_consume(&cfg) {
            Err(wait) => assert!(
                wait >= Duration::from_millis(90),
                "wait must be ~100ms (1 token / 10 per sec) when bucket exhausted, got {wait:?}"
            ),
            Ok(_) => panic!("expected Err(wait) on exhausted bucket"),
        }
    }

    /// @covers: try_consume
    #[test]
    fn test_try_consume_refill_caps_at_burst_capacity() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        // Simulate a long idle period: the refill inside try_consume must not
        // push the bucket above burst_capacity (20). After consuming one, it
        // should sit at 19, not higher.
        b.last_refill = Instant::now() - Duration::from_secs(100);
        b.try_consume(&cfg)
            .expect("refilled bucket must yield a token");
        assert!(
            (b.tokens() - 19.0).abs() < 0.001,
            "refill must cap at burst_capacity; got {}",
            b.tokens()
        );
    }

    /// @covers: try_consume
    #[test]
    fn test_try_consume_refill_restores_tokens_proportional_to_elapsed_time() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        for _ in 0..20 {
            b.try_consume(&cfg).expect("drain must succeed");
        }
        // 500ms at 10 tokens/sec = 5 tokens refilled; consuming one leaves ~4.
        b.last_refill = Instant::now() - Duration::from_millis(500);
        b.try_consume(&cfg)
            .expect("partial refill must yield a token");
        assert!(
            (b.tokens() - 4.0).abs() < 0.1,
            "expected ~4 tokens after partial refill, got {}",
            b.tokens()
        );
    }
}
