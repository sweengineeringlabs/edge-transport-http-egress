//! The object a composition root attaches to an HTTP client and inspects.

use std::fmt::Debug;

use crate::HttpCassette;

/// A record/replay layer that can be attached to a `reqwest_middleware::ClientBuilder` (it is a
/// [`reqwest_middleware::Middleware`]) and inspected through [`HttpCassette`].
///
/// Implemented for every type with those three capabilities, so an implementation package returns
/// its own private type behind `Arc<dyn CassetteLayer>` without exporting it. Attach it with
/// `ClientBuilder::with_arc`.
pub trait CassetteLayer: reqwest_middleware::Middleware + HttpCassette + Debug {}

impl<T> CassetteLayer for T where T: reqwest_middleware::Middleware + HttpCassette + Debug {}
