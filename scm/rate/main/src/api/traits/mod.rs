//! Rate API trait declarations.

pub mod processor;
pub use processor::Processor;

pub mod validator;
pub use validator::Validator;

pub mod rate_metrics;
pub use rate_metrics::RateMetrics;
