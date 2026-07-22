//! edge-transport-http-egress-loadbalancer — Client-side load-balancer middleware.
//!
//! Provides a [`LoadbalancerLayerPoolMetrics`] that plugs into
//! `reqwest_middleware::ClientBuilder` and rewrites the request URL to a healthy
//! backend selected by [`LoadbalancerConfig`] strategy (round-robin, weighted,
//! or least-connections).

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::{
    Backend, BackendConfig, BackendCountRequest, BackendCountResponse, BackendHealth, BackendId,
    BackendPoolInstance, ConfigValidationRequest, DescribeRequest, DescribeResponse,
    LoadbalancerConfig, LoadbalancerLayerPoolMetrics, LoadbalancerMiddlewareError,
    LoadbalancerSvcProcessor, Outcome, PoolError, PoolMetrics, Processor, Strategy, Validator,
};
pub use crate::saf::{PoolMetricsFactory, ProcessorFactory, ValidatorFactory};
