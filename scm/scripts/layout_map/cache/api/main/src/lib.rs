//! edge_transport_http_egress_cache_api — the HTTP-cache middleware contract.
//!
//! This crate holds what a consumer programs against: the [`CacheConfig`] schema, the error, the
//! request/response DTOs, the [`Processor`], [`Validator`] and [`HttpCache`] traits, and the
//! [`CacheFactory`] that builds them. The standard implementation is the
//! `edge-transport-http-egress-cache-std` package.

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
    ConfigValidationRequest, DescribeRequest, DescribeResponse, FallbackTtlRequest,
    FallbackTtlResponse,
};
pub use crate::error::CacheError;
pub use crate::traits::{CacheFactory, CacheLayer, HttpCache, Processor, Validator};
pub use crate::types::CacheConfig;
