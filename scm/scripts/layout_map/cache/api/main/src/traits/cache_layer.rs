//! The object a composition root attaches to an HTTP client and inspects.

use std::fmt::Debug;

use crate::HttpCache;

/// An HTTP response cache that can be attached to a `reqwest_middleware::ClientBuilder`
/// (it is a [`reqwest_middleware::Middleware`]) and inspected through [`HttpCache`].
///
/// Implemented for every type with those three capabilities, so an implementation package returns
/// its own private type behind `Arc<dyn CacheLayer>` without exporting it.
pub trait CacheLayer: reqwest_middleware::Middleware + HttpCache + Debug {}

impl<T> CacheLayer for T where T: reqwest_middleware::Middleware + HttpCache + Debug {}
