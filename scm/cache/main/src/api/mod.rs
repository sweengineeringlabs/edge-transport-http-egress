//! API layer — public schema + trait contracts + public types.

mod dto;
mod error;
mod processor;
mod traits;
mod types;

// Re-export public traits and errors at the top level
pub use error::CacheError;
pub use traits::{HttpCache, Processor, Validator};

// Re-export public DTOs at the top level
pub use dto::{
    ConfigValidationRequest, DescribeRequest, DescribeResponse, FallbackTtlRequest,
    FallbackTtlResponse,
};

// Re-export public types at the top level
pub use types::{CacheConfig, HttpCacheSvcProcessor, MiddlewareHttpCache};
