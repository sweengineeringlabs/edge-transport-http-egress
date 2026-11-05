//! edge_transport_http_egress_retry_api — the retry middleware contract.
//!
//! This crate holds what a consumer programs against: the [`RetryConfig`] schema and its builders,
//! the error, the request/response DTOs, the [`Processor`] and [`Validator`] traits, the
//! [`RetryLayer`] a processor returns, and the [`RetryFactory`] that builds them. The standard
//! implementation is the `edge-transport-http-egress-retry-std` package.

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
    DecorateRequest, DecorateResponse, ProcessorRequest, ProcessorResponse, ValidationRequest,
};
pub use crate::error::RetryError;
pub use crate::traits::{Processor, RetryFactory, RetryLayer, Validator};
pub use crate::types::{ApplicationConfigBuilder, RetryConfig, RetryConfigBuilder};
