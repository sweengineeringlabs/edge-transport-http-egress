//! Core cache layer implementation — TTL-based HTTP middleware.

#[allow(clippy::module_inception)]
mod layer;
mod request_snapshot;
mod ttl_decision;
