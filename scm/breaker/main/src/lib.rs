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

pub use crate::api::{
    Admission, AdmitRequest, AdmitResponse, BreakerConfig, BreakerError,
    BreakerLayerBreakerMetrics, BreakerMetrics, BreakerTransition, ConfigValidationRequest,
    DescribeRequest, DescribeResponse, FailureThresholdRequest, FailureThresholdResponse,
    HttpBreakerSvcProcessor, Outcome, Processor, RecordOutcomeRequest, RecordOutcomeResponse,
    Validator,
};
pub use crate::saf::{BreakerMetricsFactory, ProcessorFactory, ValidatorFactory};
