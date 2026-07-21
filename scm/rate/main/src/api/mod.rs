//! API layer — public schema + trait contracts + public types.

mod dto;
mod error;
mod traits;
mod types;

// Re-export public traits and errors at the top level
pub use error::RateError;
pub use traits::{Processor, RateMetrics, Validator};

// Re-export public DTOs at the top level
pub use dto::{
    ConfigValidationRequest, DescribeRequest, DescribeResponse, RateLimitRequest, RateLimitResponse,
};

// Re-export public types at the top level
pub use types::{HttpRateSvcProcessor, RateConfig, RateLayerRateMetrics};
