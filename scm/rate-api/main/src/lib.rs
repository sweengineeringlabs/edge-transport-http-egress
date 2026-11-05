//! edge_transport_http_egress_rate_api — the rate-limit middleware contract.
//!
//! This crate holds what a consumer programs against: the [`RateConfig`] schema, the error, the
//! request/response DTOs, the [`Processor`], [`Validator`] and [`RateMetrics`] traits, and the
//! [`RateFactory`] that builds them. The standard implementation is the
//! `edge-transport-http-egress-rate-std` package.

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
    ConfigValidationRequest, DescribeRequest, DescribeResponse, RateLimitRequest, RateLimitResponse,
};
pub use crate::error::RateError;
pub use crate::traits::{Processor, RateFactory, RateLayer, RateMetrics, Validator};
pub use crate::types::RateConfig;
