//! Primary trait contracts for `edge_transport_http_egress_breaker`.

pub mod processor;
pub mod validator;

pub use processor::Processor;
pub use validator::Validator;
pub mod breaker_metrics;
pub use breaker_metrics::BreakerMetrics;

// Moved to edge-transport-breaker-policy (ADR-004/ADR-003): this crate's canonical
// implementation (edge-transport-grpc-egress's) won the Phase 1 design review.
pub use edge_transport_breaker_policy::BreakerTransition;
