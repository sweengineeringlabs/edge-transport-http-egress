//! edge_transport_http_egress_retry — Opinionated retry middleware (wraps reqwest-retry with SWE defaults).
//!

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::types::retry::retry_config_builder::RetryConfigBuilder;
pub use crate::api::types::retry::retry_layer::RetryLayer;
pub use crate::api::{HttpRetrySvc, Processor, RetryConfig, RetryError, Validator};
