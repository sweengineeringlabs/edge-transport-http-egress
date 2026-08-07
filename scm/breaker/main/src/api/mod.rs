//! API layer — public schema + trait contracts + public types.

mod breaker;
mod dto;
mod error;
mod traits;
mod types;

// Re-export public traits and errors at the top level
pub use error::BreakerError;
pub use traits::{BreakerMetrics, BreakerTransition, Processor, Validator};

// Re-export public DTOs at the top level
pub use dto::{
    AdmitRequest, AdmitResponse, ConfigValidationRequest, DescribeRequest, DescribeResponse,
    FailureThresholdRequest, FailureThresholdResponse, RecordOutcomeRequest, RecordOutcomeResponse,
};

// Re-export public types at the top level
pub use types::{
    Admission, BreakerConfig, BreakerLayerBreakerMetrics, HttpBreakerSvcProcessor, Outcome,
};
