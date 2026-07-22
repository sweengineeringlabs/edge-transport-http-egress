//! Per-host breaker state machine.

pub(crate) mod host_breaker;
pub(crate) mod state;
pub(crate) use host_breaker::DefaultHostBreaker;
