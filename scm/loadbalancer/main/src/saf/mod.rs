//! SAF layer — public facade.
//!
//! Exposes trait factories for the loadbalancer middleware. Traits are NOT
//! re-exported from this module (SEA Rule 126). Core types are NOT re-exported
//! directly (SEA Rule 47).

mod pool_metrics_svc_factory;
mod processor_svc_factory;
mod validator_svc_factory;

pub use pool_metrics_svc_factory::PoolMetricsFactory;
pub use processor_svc_factory::ProcessorFactory;
pub use validator_svc_factory::ValidatorFactory;
