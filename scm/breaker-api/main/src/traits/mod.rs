//! Trait contracts of the breaker API.
pub mod breaker_factory;
pub mod breaker_layer;
pub mod breaker_metrics;
pub mod processor;
pub mod validator;

pub use breaker_factory::BreakerFactory;
pub use breaker_layer::BreakerLayer;
pub use breaker_metrics::BreakerMetrics;
pub use processor::Processor;
pub use validator::Validator;

// Moved to edge-transport-breaker-policy (ADR-004/ADR-003): this crate's canonical
// implementation (edge-transport-grpc-egress's) won the Phase 1 design review.
pub use edge_transport_breaker_policy::BreakerTransition;
