//! edge_transport_http_egress_retry — Opinionated retry middleware (wraps reqwest-retry with SWE defaults).
//!

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::{
    ApplicationConfigBuilder, DecorateRequest, DecorateResponse, HttpRetrySvc, Processor,
    ProcessorRequest, ProcessorResponse, RetryConfig, RetryConfigBuilder, RetryError, RetryLayer,
    ValidationRequest, Validator,
};
pub use crate::saf::{ProcessorFactory, ValidatorFactory};
