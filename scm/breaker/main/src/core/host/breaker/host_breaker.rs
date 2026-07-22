//! Per-host breaker state machine.
//!
//! Three states: **Closed** (traffic flows), **Open** (fail
//! fast), **HalfOpen** (a probe is allowed to test recovery).
//! Transitions are driven by outcome observations + time
//! (elapsed since entering Open).

use super::state::State;
use std::time::{Duration, Instant};

use crate::api::CircuitBreakerNode;
use crate::api::HostBreaker;
use crate::api::{
    Admission, AdmitRequest, AdmitResponse, BreakerError, ClosedStateRequest, ClosedStateResponse,
    HalfOpenStateRequest, HalfOpenStateResponse, OpenStateRequest, OpenStateResponse, Outcome,
    RecordRequest, RecordResponse,
};

/// Breaker state for a single host. Protected by an async
/// `Mutex` inside [`BreakerLayerBreakerMetrics`](crate::api::BreakerLayerBreakerMetrics)
/// — the mutex serializes state transitions so concurrent
/// requests to the same host see coherent state.
#[derive(Debug)]
pub(crate) struct DefaultHostBreaker {
    state: State,
    /// Consecutive failures in Closed state. Trips at
    /// `config.failure_threshold`.
    consecutive_failures: u32,
    /// Consecutive successes in HalfOpen state. Closes the
    /// breaker at `config.reset_after_successes`.
    consecutive_successes: u32,
}

impl CircuitBreakerNode for DefaultHostBreaker {
    fn admit(&mut self, request: AdmitRequest) -> Result<AdmitResponse, BreakerError> {
        let config = request.config;
        let admission = match self.state {
            State::Closed => Admission::Proceed,
            State::Open { since } => {
                let elapsed = since.elapsed();
                let wait = Duration::from_secs(config.half_open_after_seconds);
                if elapsed >= wait {
                    self.state = State::HalfOpen;
                    self.consecutive_successes = 0;
                    Admission::Proceed
                } else {
                    Admission::RejectOpen
                }
            }
            State::HalfOpen => Admission::Proceed,
        };
        Ok(AdmitResponse { admission })
    }

    fn record(&mut self, request: RecordRequest) -> Result<RecordResponse, BreakerError> {
        let config = request.config;
        match (self.state, request.outcome) {
            (State::Closed, Outcome::Success) => {
                self.consecutive_failures = 0;
            }
            (State::Closed, Outcome::Failure) => {
                self.consecutive_failures = self.consecutive_failures.saturating_add(1);
                if self.consecutive_failures >= config.failure_threshold {
                    self.state = State::Open {
                        since: Instant::now(),
                    };
                }
            }
            (State::HalfOpen, Outcome::Success) => {
                self.consecutive_successes = self.consecutive_successes.saturating_add(1);
                if self.consecutive_successes >= config.reset_after_successes {
                    self.state = State::Closed;
                    self.consecutive_failures = 0;
                    self.consecutive_successes = 0;
                }
            }
            (State::HalfOpen, Outcome::Failure) => {
                self.state = State::Open {
                    since: Instant::now(),
                };
                self.consecutive_successes = 0;
            }
            (State::Open { .. }, _) => {
                // record called while Open — caller should
                // not dispatch in this state. Ignore.
            }
        }
        Ok(RecordResponse)
    }
}

impl DefaultHostBreaker {
    pub(crate) fn new() -> Self {
        Self {
            state: super::state::State::Closed,
            consecutive_failures: 0,
            consecutive_successes: 0,
        }
    }

    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "test helper — used only by inline test assertions"
        )
    )]
    fn state(&self) -> State {
        self.state
    }
}

impl HostBreaker for DefaultHostBreaker {
    fn is_open(&self, _request: OpenStateRequest) -> Result<OpenStateResponse, BreakerError> {
        Ok(OpenStateResponse {
            value: matches!(self.state, State::Open { .. }),
        })
    }

    fn is_half_open(
        &self,
        _request: HalfOpenStateRequest,
    ) -> Result<HalfOpenStateResponse, BreakerError> {
        Ok(HalfOpenStateResponse {
            value: matches!(self.state, State::HalfOpen),
        })
    }

    fn is_closed(&self, _request: ClosedStateRequest) -> Result<ClosedStateResponse, BreakerError> {
        Ok(ClosedStateResponse {
            value: matches!(self.state, State::Closed),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn test_config() -> Arc<crate::api::BreakerConfig> {
        Arc::new(
            crate::api::BreakerConfig::from_config(
                r#"
                failure_threshold = 3
                half_open_after_seconds = 1
                reset_after_successes = 2
                failure_statuses = [500, 502, 503, 504]
            "#,
            )
            .unwrap(),
        )
    }

    fn admit(b: &mut DefaultHostBreaker, config: &Arc<crate::api::BreakerConfig>) -> Admission {
        b.admit(AdmitRequest {
            config: Arc::clone(config),
        })
        .unwrap()
        .admission
    }

    fn record(
        b: &mut DefaultHostBreaker,
        config: &Arc<crate::api::BreakerConfig>,
        outcome: Outcome,
    ) {
        b.record(RecordRequest {
            config: Arc::clone(config),
            outcome,
        })
        .unwrap();
    }

    /// @covers: new
    #[test]
    fn test_new_starts_closed() {
        let b = DefaultHostBreaker::new();
        assert_eq!(b.state(), State::Closed);
    }

    /// @covers: admit
    #[test]
    fn test_closed_admits_traffic() {
        let cfg = test_config();
        let mut b = DefaultHostBreaker::new();
        assert_eq!(admit(&mut b, &cfg), Admission::Proceed);
    }

    /// @covers: record
    #[test]
    fn test_record_failure_increments_toward_threshold() {
        let cfg = test_config();
        let mut b = DefaultHostBreaker::new();
        record(&mut b, &cfg, Outcome::Failure);
        assert_eq!(b.state(), State::Closed, "one failure stays closed");
        record(&mut b, &cfg, Outcome::Failure);
        record(&mut b, &cfg, Outcome::Failure);
        assert!(
            matches!(b.state(), State::Open { .. }),
            "three failures trip breaker"
        );
    }

    /// @covers: record
    #[test]
    fn test_failures_below_threshold_stay_closed() {
        let cfg = test_config();
        let mut b = DefaultHostBreaker::new();
        record(&mut b, &cfg, Outcome::Failure);
        record(&mut b, &cfg, Outcome::Failure);
        assert_eq!(b.state(), State::Closed);
    }

    /// @covers: record
    #[test]
    fn test_failures_at_threshold_trip_to_open() {
        let cfg = test_config();
        let mut b = DefaultHostBreaker::new();
        for _ in 0..3 {
            record(&mut b, &cfg, Outcome::Failure);
        }
        assert!(matches!(b.state(), State::Open { .. }));
    }

    /// @covers: record
    #[test]
    fn test_success_in_closed_resets_failure_counter() {
        let cfg = test_config();
        let mut b = DefaultHostBreaker::new();
        record(&mut b, &cfg, Outcome::Failure);
        record(&mut b, &cfg, Outcome::Failure);
        record(&mut b, &cfg, Outcome::Success);
        record(&mut b, &cfg, Outcome::Failure);
        record(&mut b, &cfg, Outcome::Failure);
        assert_eq!(b.state(), State::Closed);
    }

    /// @covers: admit
    #[test]
    fn test_open_rejects_before_wait_elapsed() {
        let cfg = test_config();
        let mut b = DefaultHostBreaker::new();
        for _ in 0..3 {
            record(&mut b, &cfg, Outcome::Failure);
        }
        assert_eq!(admit(&mut b, &cfg), Admission::RejectOpen);
    }

    /// @covers: admit
    #[test]
    fn test_open_promotes_to_half_open_after_wait() {
        let cfg = test_config();
        let mut b = DefaultHostBreaker::new();
        for _ in 0..3 {
            record(&mut b, &cfg, Outcome::Failure);
        }
        b.state = State::Open {
            since: Instant::now() - Duration::from_secs(2),
        };
        assert_eq!(admit(&mut b, &cfg), Admission::Proceed);
        assert_eq!(b.state(), State::HalfOpen);
    }

    /// @covers: record
    #[test]
    fn test_half_open_success_counts_toward_reset() {
        let cfg = test_config();
        let mut b = DefaultHostBreaker::new();
        b.state = State::HalfOpen;
        record(&mut b, &cfg, Outcome::Success);
        assert_eq!(b.state(), State::HalfOpen);
        record(&mut b, &cfg, Outcome::Success);
        assert_eq!(b.state(), State::Closed);
    }

    /// @covers: record
    #[test]
    fn test_half_open_failure_returns_to_open() {
        let cfg = test_config();
        let mut b = DefaultHostBreaker::new();
        b.state = State::HalfOpen;
        b.consecutive_successes = 1;
        record(&mut b, &cfg, Outcome::Failure);
        assert!(matches!(b.state(), State::Open { .. }));
    }

    /// @covers: is_open
    #[test]
    fn test_is_open_false_when_closed() {
        let b = DefaultHostBreaker::new();
        assert!(
            !b.is_open(OpenStateRequest).unwrap().value,
            "new breaker starts closed, not open"
        );
    }

    /// @covers: is_open
    #[test]
    fn test_is_open_true_when_tripped() {
        let cfg = test_config();
        let mut b = DefaultHostBreaker::new();
        for _ in 0..3 {
            record(&mut b, &cfg, Outcome::Failure);
        }
        assert!(
            b.is_open(OpenStateRequest).unwrap().value,
            "breaker must be open after threshold failures"
        );
    }

    /// @covers: is_open
    #[test]
    fn test_is_open_false_when_half_open() {
        let mut b = DefaultHostBreaker::new();
        b.state = State::HalfOpen;
        assert!(
            !b.is_open(OpenStateRequest).unwrap().value,
            "HalfOpen is not Open"
        );
    }

    /// @covers: state
    #[test]
    fn test_state_returns_correct_variant() {
        let mut b = DefaultHostBreaker::new();
        assert_eq!(b.state(), State::Closed);
        b.state = State::HalfOpen;
        assert_eq!(b.state(), State::HalfOpen);
        b.state = State::Open {
            since: Instant::now(),
        };
        assert!(matches!(b.state(), State::Open { .. }));
    }
}
