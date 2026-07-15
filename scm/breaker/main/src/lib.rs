//! edge_transport_http_egress_breaker — Circuit-breaker middleware — fail fast on degraded upstreams.
//!

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
// `unwrap`/`expect` are denied in production code (Cargo.toml `[lints.clippy]`)
// but are the idiomatic assertion mechanism in inline `#[cfg(test)]` modules.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::types::admission::Admission;
pub use crate::api::types::breaker::breaker_layer::BreakerLayer;
pub use crate::api::types::outcome::Outcome;
pub use crate::api::{
    BreakerConfig, BreakerError, BreakerMetrics, CircuitBreakerNode, HttpBreakerSvc, Processor,
    Validator,
};
pub use saf::get_failure_threshold;
