//! edge-transport-http-egress-loadbalancer — Client-side load-balancer middleware.
//!
//! Provides a [`LoadbalancerLayer`] that plugs into `reqwest_middleware::ClientBuilder`
//! and rewrites the request URL to a healthy backend selected by [`LoadbalancerConfig`]
//! strategy (round-robin, weighted, or least-connections).

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use crate::api::types::{
    Backend, BackendConfig, BackendHealth, BackendId, BackendPoolInstance, LoadbalancerConfig,
    LoadbalancerLayer, LoadbalancerSvc, Outcome, PoolError, Strategy,
};
pub use crate::api::{LoadbalancerMiddlewareError, Processor, Validator};
pub use saf::{build_loadbalancer_layer, validate_loadbalancer_config};
