//! SAF layer — public facade.

mod processor_svc_factory;
mod rate_metrics_svc_factory;
mod validator_svc_factory;

pub use processor_svc_factory::ProcessorFactory;
pub use rate_metrics_svc_factory::RateMetricsFactory;
pub use validator_svc_factory::ValidatorFactory;
