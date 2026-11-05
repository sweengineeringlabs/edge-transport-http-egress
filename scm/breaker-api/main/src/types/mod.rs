//! Value objects for the breaker API.
pub mod breaker;

pub use breaker::breaker_config::BreakerConfig;
// Moved to edge-transport-breaker-policy (ADR-004/ADR-003).
pub use edge_transport_breaker_policy::{Admission, Outcome};
