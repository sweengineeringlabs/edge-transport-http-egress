//! edge_transport_http_egress_cassette_api — the record/replay middleware contract.
//!
//! This crate holds what a consumer programs against: the [`CassetteConfig`] schema and its
//! builder, the error, the request/response DTOs, the [`Processor`], [`Validator`] and
//! [`HttpCassette`] traits, the [`CassetteLayer`] a factory returns, and the [`CassetteFactory`]
//! that builds them. The standard implementation is the `edge-transport-http-egress-cassette-std`
//! package.

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
    CassetteModeRequest, CassetteModeResponse, ConfigValidationRequest, DescribeRequest,
    DescribeResponse,
};
pub use crate::error::CassetteError;
pub use crate::traits::{CassetteFactory, CassetteLayer, HttpCassette, Processor, Validator};
pub use crate::types::{CassetteConfig, CassetteConfigBuilder};
