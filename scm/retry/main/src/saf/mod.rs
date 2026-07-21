//! SAF layer — public facade.
//!
//! Exposes trait factories for the retry middleware. Traits are NOT
//! re-exported from this module; core types are NOT re-exported directly.

mod processor_svc_factory;
mod validator_svc_factory;

pub use processor_svc_factory::ProcessorFactory;
pub use validator_svc_factory::ValidatorFactory;
