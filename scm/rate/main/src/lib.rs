//! edge_transport_http_egress_rate — Client-side rate-limiter middleware — token bucket per host.
//!

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::{
    ConfigValidationRequest, DescribeRequest, DescribeResponse, HttpRateSvcProcessor, Processor,
    RateConfig, RateError, RateLayerRateMetrics, RateLimitRequest, RateLimitResponse, RateMetrics,
    Validator,
};
pub use crate::saf::{ProcessorFactory, RateMetricsFactory, ValidatorFactory};
