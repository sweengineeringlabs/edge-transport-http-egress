//! Interface contract for the default HTTP egress implementation.

use crate::HttpEgress;

/// Dyn-safe alias for the default (reqwest-backed) HTTP outbound interface.
///
/// Callers that want a trait object of the default HTTP outbound use
/// `Arc<HttpEgressObject>` instead of `Arc<dyn HttpEgress>`.
pub type HttpEgressObject = dyn HttpEgress;
