//! The object a composition root attaches to an HTTP client and inspects.

use std::fmt::Debug;

use crate::RateMetrics;

/// A per-host rate limiter that can be attached to a `reqwest_middleware::ClientBuilder`
/// (it is a [`reqwest_middleware::Middleware`]) and inspected through [`RateMetrics`].
///
/// Implemented for every type with those three capabilities, so an implementation package returns
/// its own private type behind `Arc<dyn RateLayer>` without exporting it.
pub trait RateLayer: reqwest_middleware::Middleware + RateMetrics + Debug {}

impl<T> RateLayer for T where T: reqwest_middleware::Middleware + RateMetrics + Debug {}
