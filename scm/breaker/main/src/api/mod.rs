//! API layer — public schema + trait contracts + public types.

mod breaker;
mod dto;
mod error;
mod traits;
mod types;

// Re-export public traits and errors at the top level
pub use error::BreakerError;
pub use traits::{BreakerMetrics, CircuitBreakerNode, HostBreaker, Processor, Validator};

// Re-export public DTOs at the top level
pub use dto::{
    AdmitRequest, AdmitResponse, ClosedStateRequest, ClosedStateResponse, ConfigValidationRequest,
    DescribeRequest, DescribeResponse, FailureThresholdRequest, FailureThresholdResponse,
    HalfOpenStateRequest, HalfOpenStateResponse, OpenStateRequest, OpenStateResponse,
    RecordRequest, RecordResponse,
};

// Re-export public types at the top level
pub use types::{
    Admission, BreakerConfig, BreakerLayerBreakerMetrics, HttpBreakerSvcProcessor, Outcome,
};
