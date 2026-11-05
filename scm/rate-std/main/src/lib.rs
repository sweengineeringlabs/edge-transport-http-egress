//! edge_transport_http_egress_rate_std — the standard rate-limit middleware.
//!
//! The only public item is [`StdRateFactory`], the implementation of
//! `edge_transport_http_egress_rate_api::RateFactory`. Every concrete type is crate-private;
//! consumers hold the traits from the `-api` package.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
// `unwrap`/`expect` are denied in production code (Cargo.toml `[lints.clippy]`)
// but are the idiomatic assertion mechanism in inline `#[cfg(test)]` modules.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod core;
mod saf;

pub use crate::saf::StdRateFactory;
