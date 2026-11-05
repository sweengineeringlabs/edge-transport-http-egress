//! The object a composition root attaches to an HTTP client.

use std::fmt::Debug;

/// A retry policy that can be attached to a `reqwest_middleware::ClientBuilder` (it is a
/// [`reqwest_middleware::Middleware`]).
///
/// Implemented for every type with those capabilities, so an implementation package returns its
/// own private type behind `Arc<dyn RetryLayer>` without exporting it. Attach it with
/// `ClientBuilder::with_arc`.
pub trait RetryLayer: reqwest_middleware::Middleware + Debug {}

impl<T> RetryLayer for T where T: reqwest_middleware::Middleware + Debug {}
