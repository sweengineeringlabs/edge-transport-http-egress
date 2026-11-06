//! The object a composition root attaches to an HTTP client and inspects.

use std::fmt::Debug;

use crate::BreakerMetrics;

/// A per-host circuit breaker that can be attached to a `reqwest_middleware::ClientBuilder`
/// (it is a [`reqwest_middleware::Middleware`]) and inspected through [`BreakerMetrics`].
///
/// Implemented for every type with those three capabilities, so an implementation package returns
/// its own private type behind `Arc<dyn BreakerLayer>` without exporting it.
pub trait BreakerLayer: reqwest_middleware::Middleware + BreakerMetrics + Debug {}

impl<T> BreakerLayer for T where T: reqwest_middleware::Middleware + BreakerMetrics + Debug {}
