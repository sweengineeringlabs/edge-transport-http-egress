//! edge_transport_http_egress_breaker_api — the circuit-breaker middleware contract.
//!
//! This crate holds what a consumer programs against: the [`BreakerConfig`] schema, the error,
//! the request/response DTOs, the [`Processor`], [`Validator`] and [`BreakerMetrics`] traits, and
//! the [`BreakerFactory`] that builds them. The standard implementation is the
//! `edge-transport-http-egress-breaker-std` package.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
// `unwrap`/`expect` are denied in production code (Cargo.toml `[lints.clippy]`)
// but are the idiomatic assertion mechanism in inline `#[cfg(test)]` modules.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod dto;
mod error;
mod traits;
mod types;

pub use crate::dto::{
    AdmitRequest, AdmitResponse, ConfigValidationRequest, DescribeRequest, DescribeResponse,
    FailureThresholdRequest, FailureThresholdResponse, RecordOutcomeRequest, RecordOutcomeResponse,
};
pub use crate::error::BreakerError;
pub use crate::traits::{
    BreakerFactory, BreakerLayer, BreakerMetrics, BreakerTransition, Processor, Validator,
};
pub use crate::types::{Admission, BreakerConfig, Outcome};
