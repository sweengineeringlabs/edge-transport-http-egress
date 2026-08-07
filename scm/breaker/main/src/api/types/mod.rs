//! Value objects for the breaker API.

pub(crate) mod application_config_builder;
pub mod breaker;
pub(crate) mod http_breaker_svc_processor;

pub use breaker::breaker_config::BreakerConfig;
pub use breaker::breaker_layer_breaker_metrics::BreakerLayerBreakerMetrics;
pub use http_breaker_svc_processor::HttpBreakerSvcProcessor;

// Moved to edge-transport-breaker-policy (ADR-004/ADR-003).
pub use edge_transport_breaker_policy::{Admission, Outcome};
